extern crate sdl2;
extern crate cgmath;

use camera::Camera;
use mesh::Mesh;
use bitmap::Bitmap;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::EventPump;

use cgmath::EuclideanVector;
use cgmath::Point;
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
    bitmap: Bitmap,
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
        self.back_buffer
            .with_lock(None, |buffer: &mut [u8], _: usize| {
                for i in 0..slice.len() {
                    buffer[i] = slice[i];
                }
            })
            .unwrap();
    }

    fn draw_point(&mut self, point: Vector2<u32>) {
        if point.x < self.bitmap.width() && point.y < self.bitmap.height() {
            self.bitmap.set_pixel(point, Color::RGB(255, 255, 0));
        }
    }

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
            let cam_from_origin = -cam.position.to_vec();
            let world_mat = Matrix4::from_translation(cam_from_origin) *
                            Matrix4::from_translation(mesh.position) *
                            Matrix4::from(Matrix3::from_euler(cgmath::rad(mesh.rotation.x),
                                                              cgmath::rad(mesh.rotation.y),
                                                              cgmath::rad(mesh.rotation.z)));

            let mat = projection_mat * view_mat * world_mat;

            for i in 0..(mesh.vertices.len() - 1) {
                let point1 = self.project(mesh.vertices[i], mat);
                let point2 = self.project(mesh.vertices[i + 1], mat);

                self.draw_line(point1.cast(), point2.cast());
            }

            //            for vertex in mesh.vertices.iter() {
            //                let point = self.project(*vertex, mat);
            //
            //                // println!("projected point = {:?}", point);
            //
            //                self.set_pixel(point);
            //            }
        }
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
        self.bitmap.clear(color);
    }

    pub fn present(&mut self) {
        self.copy_bitmap_to_back_buffer();
        self.renderer.copy(&self.back_buffer, None, None);
        self.renderer.present();
    }
}
