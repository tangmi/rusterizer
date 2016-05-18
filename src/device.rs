extern crate sdl2;
extern crate cgmath;

use std::mem;
use std::f64;
use std::cmp;

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
use cgmath::BaseNum;
use cgmath::EuclideanVector;

use rect::Point;
use rect::Rect;

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

            // println!("vertices = {:?}", mesh.vertices.len());
            // println!("faces = {:?}", mesh.faces.len());

            for (i, face) in mesh.faces.iter().enumerate() {
                // let color = {
                //     let color_val = 0.25 + (i % faces_count) as f64 * 0.75 / faces_count as f64;
                //     let color_val_u8 = (color_val * 255.0) as u8;
                //     Color::RGB(color_val_u8, 255 - color_val_u8, 0)
                // };
                let color = Color::RGB(255, 255, 255);

                let pixel_a = self.project(mesh.vertices[face.a], mat);
                let pixel_b = self.project(mesh.vertices[face.b], mat);
                let pixel_c = self.project(mesh.vertices[face.c], mat);

                let p0 = Vector2::new(pixel_a.x, pixel_a.y).cast();
                let p1 = Vector2::new(pixel_b.x, pixel_b.y).cast();
                let p2 = Vector2::new(pixel_c.x, pixel_c.y).cast();
                // self.draw_line(p0, p1, color);
                // self.draw_line(p1, p2, color);
                // self.draw_line(p2, p0, color);

                let normal = (mesh.vertices[face.c] - mesh.vertices[face.a])
                    .cross(mesh.vertices[face.b] - mesh.vertices[face.a])
                    .normalize();

                self.draw_triangle(p0, p1, p2, color);

                // self.draw_triangle(pixel_a, pixel_b, pixel_c, color);
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
        // Device::copy_bitmap_to_texture(&self.depth_buffer, &mut self.texture);
        Device::copy_bitmap_to_texture(&self.back_buffer, &mut self.texture);
        self.renderer.copy(&self.texture, None, None);
        self.renderer.present();
    }

    fn project(&self, vertex: Vector3<f64>, mat: Matrix4<f64>) -> Vector3<f64> {
        let point = mat * vertex.extend(1.0);

        let width = self.width() as f64;
        let height = self.height() as f64;

        Vector3::new(point.x * width + width / 2.0,
                     -point.y * height + height / 2.0,
                     point.z)
    }

    fn width(&self) -> i32 {
        self.back_buffer.width() as i32
    }

    fn height(&self) -> i32 {
        self.back_buffer.height() as i32
    }







    pub fn test_draw_triangles(&mut self) {
        let vec1 = vec!{Vector2::new(10, 70),   Vector2::new(50, 160),  Vector2::new(70, 80)};
        self.draw_triangle(vec1[0], vec1[1], vec1[2], Color::RGB(255, 0, 0));

        let vec2 = vec!{Vector2::new(180, 50),  Vector2::new(150, 1),   Vector2::new(70, 180)};
        self.draw_triangle(vec2[0], vec2[1], vec2[2], Color::RGB(255, 255, 255));

        let vec3 = vec!{Vector2::new(180, 150), Vector2::new(120, 160), Vector2::new(130, 180)};
        self.draw_triangle(vec3[0], vec3[1], vec3[2], Color::RGB(0, 255, 0));




        let vec0 = vec!{Vector2::new(10, 70),  Vector2::new(5, 120), Vector2::new(50, 160)};
        self.draw_triangle(vec0[0], vec0[1], vec0[2], Color::RGB(100, 100, 0));
    }






    fn set_pixel(&mut self, point: Point, z: f64, color: Color) {
        if point.x < self.back_buffer.width() as i32
            && point.x >= 0
            && point.y < self.back_buffer.height() as i32
            && point.y >= 0 {

            let upoint = point.cast();

            if self.depth_buffer.get_pixel(upoint) < z {
                return;
            }

            self.depth_buffer.set_pixel(upoint, z);
            self.back_buffer.set_pixel(upoint, Rgb24::from(color));
        }
    }

    /// taken from https://en.wikipedia.org/wiki/Bresenham's_line_algorithm
    fn draw_line(&mut self, pt0: Point, pt1: Point, color: Color) {
        let mut x0 = pt0.x as i32;
        let mut y0 = pt0.y as i32;
        let mut x1 = pt1.x as i32;
        let mut y1 = pt1.y as i32;

        let steep = {
            if (x0 - x1).abs() < (y0 - y1).abs() {
                // if the line is steep, transpose image
                mem::swap(&mut x0, &mut y0);
                mem::swap(&mut x1, &mut y1);
                true
            } else {
                false
            }
        };

        if x0 > x1 {
            // make it so our algorithm goes left to right always
            mem::swap(&mut x0, &mut x1);
            mem::swap(&mut y0, &mut y1);
        }

        let dx = x1 - x0;
        let dy = y1 - y0;
        let sgn_dy = dy.signum();
        let derr2 = dy.abs() * 2;
        let mut err2 = 0;
        let mut y = y0;
        for x in x0..x1 {
            if steep {
                self.set_pixel(Vector2::new(y, x), 0.0, color);
            } else {
                self.set_pixel(Vector2::new(x, y), 0.0, color);
            }

            err2 += derr2;
            if err2 > dx {
                y += sgn_dy;
                err2 -= dx * 2;
            }
        }
    }

    fn draw_triangle(&mut self, pt0: Point, pt1: Point, pt2: Point, color: Color)
    {
        let pts = vec![pt0, pt1, pt2];
        let bounds = Rect::from_bounding(&pts);
        let window_bounds = Rect::new(Point::new(0, 0), Point::new(self.width(), self.height()));

        match bounds.intersect(window_bounds) {
            Option::Some(clipped) => {
                for y in clipped.top..clipped.bottom {
                    for x in clipped.left..clipped.right {
                        let pt = Point::new(x, y);
                        if Device::is_inside_triangle(pt, pt0, pt1, pt2) {
                            self.set_pixel(pt, 0_f64, color);
                        }
                    }
                }
            },
            Option::None => {},
        }
    }

    ///! returns if a point pt is inside a triangle given by pt0, pt1, and pt2
    fn is_inside_triangle(pt: Point, pt0: Point, pt1: Point, pt2: Point) -> bool {
        let u : Vector3<f64> = Vector3::new(
            pt2.x - pt0.x,
            pt1.x - pt0.x,
            pt0.x - pt.x
        ).cross(Vector3::new(
            pt2.y - pt0.y,
            pt1.y - pt0.y,
            pt0.y - pt.y
        )).cast();

        let barycentric = if u.z.abs() < 1.0 {
            // triangle is degenerate, in this case return smth with negative coordinates
            (-1.0, 1.0, 1.0)
        } else {
            (1.0 - (u.x + u.y) / u.z,
             u.y / u.z,
             u.x / u.z)
        };

        barycentric.0 >= 0.0 && barycentric.1 >= 0.0 && barycentric.2 >= 0.0
    }
}
