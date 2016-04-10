extern crate sdl2;
extern crate cgmath;

use std::mem;
use std::f64;

use camera::Camera;
use mesh::Mesh;
use bitmap::Bitmap;
use bitmap::pixel_format::Rgb24;
use bitmap::pixel_format::TransferToRgb;
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
    
    pub fn clear(&mut self, color: Color) {
        self.back_buffer.clear(Rgb24::from(color));
        self.depth_buffer.clear(f64::MAX);
    }

    fn copy_bitmap_to_texture<T>(src_bitmap: &Bitmap<T>, dest_texture: &mut Texture)
        where T: Copy + Default + TransferToRgb
    {
        let slice = src_bitmap.slice();
        let width = src_bitmap.width();
        let height = src_bitmap.height();

        dest_texture.with_lock(None, |buffer: &mut [u8], _: usize| {
                        for y in 0..height {
                            for x in 0..width {
                                let offset = (y * width + x) as usize;
                                let woffset = offset * 3;
                                let (r, g, b) = slice[offset].transfer();

                                buffer[woffset + 0] = r;
                                buffer[woffset + 1] = g;
                                buffer[woffset + 2] = b;
                            }
                        }
                    })
                    .unwrap();
    }

    pub fn present(&mut self) {
        Device::copy_bitmap_to_texture(&self.depth_buffer, &mut self.texture);
        //    	Device::copy_bitmap_to_texture(&self.back_buffer, &mut self.texture);
        self.renderer.copy(&self.texture, None, None);
        self.renderer.present();
    }
}
