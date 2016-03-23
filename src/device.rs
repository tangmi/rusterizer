extern crate sdl2;
extern crate cgmath;

use std::mem;
use std::f64;

use camera::Camera;
use mesh::Mesh;
use bitmap::Bitmap;
use bitmap::pixel_format::Rgb24;
use math::Interpolate;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::EventPump;

use cgmath::Vector2;
use cgmath::Vector3;
use cgmath::Matrix3;
use cgmath::Matrix4;
use cgmath::Rotation3;

// #[derive(Debug)]
pub struct Device<'a> {
    // sdl_context: Sdl,
    // video_subsystem: VideoSubsystem,
    // window: Window,
    renderer: Renderer<'a>,
    texture: Texture,
    back_buffer: Bitmap<Rgb24>,
    depth_buffer: Bitmap<f64>,
    event_pump: EventPump,
}

#[derive(Debug)]
pub enum EventPumpAction {
    Quit,
    Continue,
}

impl<'a> Device<'a> {
    pub fn new(title: &str, width: u32, height: u32) -> Device<'a> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window(title, width, height)
                                    .position_centered()
                                    .build()
                                    .unwrap();

        let renderer = window.renderer().build().unwrap();

        let texture = renderer.create_texture_streaming(PixelFormatEnum::RGB24, width, height)
                              .unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        Device {
            renderer: renderer,
            texture: texture,
            back_buffer: Bitmap::new(width, height),
            depth_buffer: Bitmap::new(width, height),
            event_pump: event_pump,
        }
    }

    pub fn poll_events(&mut self) -> EventPumpAction {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return EventPumpAction::Quit;
                }
                _ => {}
            }
        }
        EventPumpAction::Continue
    }

    fn draw_point(&mut self, point: Vector3<f64>, color: Color) {
        let z = point.z;
        let point: Vector2<u32> = point.truncate().cast();

        if point.x < self.back_buffer.width() && point.y < self.back_buffer.height() {
            self.set_pixel(point, z, color);
        }
    }

    fn set_pixel(&mut self, point: Vector2<u32>, z: f64, color: Color) {
        // if point.x < self.back_buffer.width() && point.y < self.back_buffer.height() {
        if self.depth_buffer.get_pixel(point) < z {
            return;
        }

        self.depth_buffer.set_pixel(point, z);
        self.back_buffer.set_pixel(point, Rgb24::from(color));
        // }
    }

    pub fn render(&mut self, cam: &Camera, meshes: Vec<&Mesh>) {
        let view_mat = Matrix4::look_at(cam.position, cam.target, Vector3::unit_y());

        let projection_mat = cgmath::perspective(cgmath::rad(0.78),
                                                 (self.back_buffer.width() as f64) /
                                                 (self.back_buffer.height() as f64),
                                                 0.01,
                                                 1.0);

        for mesh in meshes {
            let world_mat = Matrix4::from_translation(mesh.position) *
                            Matrix4::from(Matrix3::from_euler(cgmath::rad(mesh.rotation.x),
                                                              cgmath::rad(mesh.rotation.y),
                                                              cgmath::rad(mesh.rotation.z)));

            let mat = projection_mat * view_mat * world_mat;

            let faces_count = mesh.faces.len();

            for (i, face) in mesh.faces.iter().enumerate() {
                let color = {
                    let color_val = 0.25 + (i % faces_count) as f64 * 0.75 / faces_count as f64;
                    let color_val_u8 = (color_val * 255.0) as u8;
                    Color::RGB(color_val_u8, 255 - color_val_u8, 0)
                };

                let pixel_a = self.project(mesh.vertices[face.a], mat);
                let pixel_b = self.project(mesh.vertices[face.b], mat);
                let pixel_c = self.project(mesh.vertices[face.c], mat);

                self.draw_triangle(pixel_a, pixel_b, pixel_c, color);
            }
        }
    }

    fn sort_points(p1: &mut Vector3<f64>, p2: &mut Vector3<f64>, p3: &mut Vector3<f64>) {
        if p1.y > p2.y {
            mem::swap(p1, p2);
        }

        if p2.y > p3.y {
            mem::swap(p2, p3);
        }

        if p1.y > p2.y {
            mem::swap(p1, p2);
        }
    }

    fn draw_triangle(&mut self,
                     p1: Vector3<f64>,
                     p2: Vector3<f64>,
                     p3: Vector3<f64>,
                     color: Color) {
        let mut p1 = p1;
        let mut p2 = p2;
        let mut p3 = p3;
        Device::sort_points(&mut p1, &mut p2, &mut p3);

        // inverse slopes
        let dinv_p1p2 = if p2.y - p1.y > 0.0 {
            (p2.x - p1.x) / (p2.y - p1.y)
        } else {
            0.0
        };

        let dinv_p1p3 = if p3.y - p1.y > 0.0 {
            (p3.x - p1.x) / (p3.y - p1.y)
        } else {
            0.0
        };

        if dinv_p1p2 > dinv_p1p3 {
            // First case where triangles are like that:
            // P1
            // -
            // --
            // - -
            // -  -
            // -   - P2
            // -  -
            // - -
            // -
            // P3
            for y in (p1.y as u32)..(p3.y as u32 + 1) {
                if (y as f64) < p2.y {
                    self.process_scan_line(y, p1, p3, p1, p2, color);
                } else {
                    self.process_scan_line(y, p1, p3, p2, p3, color);
                }
            }
        } else {
            // First case where triangles are like that:
            //       P1
            //        -
            //       --
            //      - -
            //     -  -
            // P2 -   -
            //     -  -
            //      - -
            //        -
            //       P3
            for y in (p1.y as u32)..(p3.y as u32 + 1) {
                if (y as f64) < p2.y {
                    self.process_scan_line(y, p1, p2, p1, p3, color);
                } else {
                    self.process_scan_line(y, p2, p3, p1, p3, color);
                }
            }
        }
    }

    /// drawing line between 2 points from left to right
    /// papb -> pcpd
    /// pa, pb, pc, pd must then be sorted before
    fn process_scan_line(&mut self,
                         y: u32,
                         pa: Vector3<f64>,
                         pb: Vector3<f64>,
                         pc: Vector3<f64>,
                         pd: Vector3<f64>,
                         color: Color) {
        let y = y as f64;

        let grad1 = if pa.y != pb.y {
            (y - pa.y) / (pb.y - pa.y)
        } else {
            1.0
        };

        let grad2 = if pc.y != pd.y {
            (y - pc.y) / (pd.y - pc.y)
        } else {
            1.0
        };

        let sx = f64::interpolate(pa.x, pb.x, grad1) as i32;
        let ex = f64::interpolate(pc.x, pd.x, grad2) as i32;

        // starting Z & ending Z
        let z1 = f64::interpolate(pa.z, pb.z, grad1);
        let z2 = f64::interpolate(pc.z, pd.z, grad2);

        for x in sx..ex {
            let grad = (x - sx) as f64 / (ex - sx) as f64;
            let z = f64::interpolate(z1, z2, grad);

            self.draw_point(Vector3::new(x as f64, y, z), color);
        }
    }

    fn project(&self, vertex: Vector3<f64>, mat: Matrix4<f64>) -> Vector3<f64> {
        let point = mat * vertex.extend(1.0);

        let width = self.back_buffer.width() as f64;
        let height = self.back_buffer.height() as f64;

        // println!("y coord = {}", -point.y * height + height / 2.0);

        Vector3::new(point.x * width + width / 2.0,
                     -point.y * height + height / 2.0,
                     point.z)
    }

    pub fn clear(&mut self, color: Color) {
        self.back_buffer.clear(Rgb24::from(color));
        self.depth_buffer.clear(f64::MAX);
    }

    fn copy_back_buffer_to_texture(&mut self) {
        let slice = self.back_buffer.slice();
        let pixel_width = self.back_buffer.pixel_width();

        let width = self.back_buffer.width();
        let height = self.back_buffer.height();

        self.texture
            .with_lock(None, |buffer: &mut [u8], _: usize| {
                for y in 0..height {
                    for x in 0..width {
                        let offset = (y * width + x) as usize;
                        let woffset = offset * pixel_width;
                        let rgb24 = slice[offset];

                        buffer[woffset + 0] = rgb24.r;
                        buffer[woffset + 1] = rgb24.g;
                        buffer[woffset + 2] = rgb24.b;
                    }
                }
            })
            .unwrap();
    }

    pub fn present(&mut self) {
        self.copy_back_buffer_to_texture();
        self.renderer.copy(&self.texture, None, None);
        self.renderer.present();
    }
}
