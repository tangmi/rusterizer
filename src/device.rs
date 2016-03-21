extern crate sdl2;
extern crate cgmath;

use camera::Camera;
use mesh::Mesh;
use bitmap::Bitmap;
use bitmap::pixel_format::Rgb24;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::EventPump;

use cgmath::EuclideanVector;
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
    back_buffer: Texture,
    bitmap: Bitmap<Rgb24>,
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

        let back_buffer = renderer.create_texture_streaming(PixelFormatEnum::RGB24, width, height)
                                  .unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        Device {
            renderer: renderer,
            back_buffer: back_buffer,
            bitmap: Bitmap::new(width, height),
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

    fn copy_bitmap_to_back_buffer(&mut self) {
        let slice = self.bitmap.slice();
        let pixel_width = self.bitmap.pixel_width();
        
        let width = self.bitmap.width();
        let height = self.bitmap.height();
        
        self.back_buffer
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

    fn draw_point(&mut self, point: Vector2<u32>) {
        if point.x < self.bitmap.width() && point.y < self.bitmap.height() {
            self.bitmap.set_pixel(point, Rgb24::from(Color::RGB(255, 255, 0)));
        }
    }

    /// draw a line with Bresenham's algorithm
    fn draw_bline(&mut self, point0: Vector2<f32>, point1: Vector2<f32>) {
        let mut x0 = point0.x as i32;
        let mut y0 = point0.y as i32;

        let x1 = point1.x as i32;
        let y1 = point1.y as i32;

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();

        let sx = (x1 - x0).signum();
        let sy = (y1 - y0).signum();

        let mut err = dx - dy;

        loop {
            self.draw_point(Vector2::new(x0, y0).cast());

            if (x0 == x1) && (y0 == y1) {
                return;
            }

            let err_2 = 2 * err;

            if err_2 > -dy {
                err -= dy;
                x0 += sx;
            }

            if err_2 < dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    #[allow(dead_code)]
    /// recursive draw line
    fn draw_line(&mut self, point1: Vector2<f32>, point2: Vector2<f32>) {
        let diff = point2 - point1;
        let dist = diff.length();

        if dist < 2.0 {
            return;
        }

        let midpoint = point1 + diff / 2.0;

        self.draw_point(midpoint.cast());

        self.draw_line(point1, midpoint);
        self.draw_line(midpoint, point2);
    }

    pub fn render(&mut self, cam: &Camera, meshes: Vec<&Mesh>) {
        let view_mat = Matrix4::look_at(cam.position, cam.target, Vector3::unit_y());

        let projection_mat = cgmath::perspective(cgmath::rad(0.78),
                                                 (self.bitmap.width() as f64) /
                                                 (self.bitmap.height() as f64),
                                                 0.01,
                                                 1.0);

        for mesh in meshes {
            let world_mat = Matrix4::from_translation(mesh.position) *
                            Matrix4::from(Matrix3::from_euler(cgmath::rad(mesh.rotation.x),
                                                              cgmath::rad(mesh.rotation.y),
                                                              cgmath::rad(mesh.rotation.z)));

            let mat = projection_mat * view_mat * world_mat;

            for face in &mesh.faces {
                let vert_a = mesh.vertices[face.a];
                let vert_b = mesh.vertices[face.b];
                let vert_c = mesh.vertices[face.c];

                let pixel_a = self.project(vert_a, mat).cast();
                let pixel_b = self.project(vert_b, mat).cast();
                let pixel_c = self.project(vert_c, mat).cast();

                self.draw_bline(pixel_a, pixel_b);
                self.draw_bline(pixel_b, pixel_c);
                self.draw_bline(pixel_c, pixel_a);
            }
        }
    }

    fn draw_triangle(p1: Vector3<f64>, p2: Vector3<f64>, p3: Vector3<f64>, color: Color) {
        unimplemented!();
    }

    /// drawing line between 2 points from left to right
    /// papb -> pcpd
    /// pa, pb, pc, pd must then be sorted before
    fn process_scan_line(y: u32,
                         pa: Vector3<f64>,
                         pb: Vector3<f64>,
                         pc: Vector3<f64>,
                         pd: Vector3<f64>,
                         color: Color) {
        unimplemented!();
    }

    fn project(&self, vertex: Vector3<f64>, mat: Matrix4<f64>) -> Vector2<u32> {
        let point = mat * vertex.extend(1.0);

        let width = self.bitmap.width() as f64;
        let height = self.bitmap.height() as f64;

        // println!("y coord = {}", -point.y * height + height / 2.0);

        Vector2::new((point.x * width + width / 2.0) as u32,
                     (-point.y * height + height / 2.0) as u32)
    }

    pub fn clear(&mut self, color: Color) {
        self.bitmap.clear(Rgb24::from(color));
    }

    pub fn present(&mut self) {
        self.copy_bitmap_to_back_buffer();
        self.renderer.copy(&self.back_buffer, None, None);
        self.renderer.present();
    }
}
