#![feature(alloc_system)]
extern crate alloc_system;

extern crate sdl2;
extern crate cgmath;
extern crate time;

mod bitmap;
mod device;
mod camera;
mod mesh;

use device::{Device, EventPumpAction};
use camera::Camera;

use sdl2::pixels::Color;

use cgmath::Vector;
use cgmath::Vector3;

use mesh::Mesh;
use mesh::Face;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

fn main() {
    let mut device = Device::new("soft-renderer", WIDTH, HEIGHT);

    let mut ticks = 0;

    let cam = Camera::new();

    let mut mesh_cube = Mesh::new("cube",
                                  vec![Vector3::new(-0.1, 0.1, 0.1),
                                       Vector3::new(0.1, 0.1, 0.1),
                                       Vector3::new(-0.1, -0.1, 0.1),
                                       Vector3::new(0.1, -0.1, 0.1),
                                       Vector3::new(-0.1, 0.1, -0.1),
                                       Vector3::new(0.1, 0.1, -0.1),
                                       Vector3::new(0.1, -0.1, -0.1),
                                       Vector3::new(-0.1, -0.1, -0.1)],
                                  vec![Face::new(0, 1, 2),
                                       Face::new(1, 2, 3),
                                       Face::new(1, 3, 6),
                                       Face::new(1, 5, 6),
                                       Face::new(0, 1, 4),
                                       Face::new(1, 4, 5),
                                       Face::new(2, 3, 7),
                                       Face::new(3, 6, 7),
                                       Face::new(0, 2, 7),
                                       Face::new(0, 4, 7),
                                       Face::new(4, 5, 6),
                                       Face::new(4, 6, 7)]);

    let mut mesh_cube2 = Mesh::new("cube",
                                   vec![Vector3::new(-0.05, 0.05, 0.05),
                                        Vector3::new(0.05, 0.05, 0.05),
                                        Vector3::new(-0.05, -0.05, 0.05),
                                        Vector3::new(0.05, -0.05, 0.05),
                                        Vector3::new(-0.05, 0.05, -0.05),
                                        Vector3::new(0.05, 0.05, -0.05),
                                        Vector3::new(0.05, -0.05, -0.05),
                                        Vector3::new(-0.05, -0.05, -0.05)],
                                   vec![Face::new(0, 1, 2),
                                        Face::new(1, 2, 3),
                                        Face::new(1, 3, 6),
                                        Face::new(1, 5, 6),
                                        Face::new(0, 1, 4),
                                        Face::new(1, 4, 5),
                                        Face::new(2, 3, 7),
                                        Face::new(3, 6, 7),
                                        Face::new(0, 2, 7),
                                        Face::new(0, 4, 7),
                                        Face::new(4, 5, 6),
                                        Face::new(4, 6, 7)]);


    mesh_cube.set_position(Vector3::zero());

    'running: loop {
        let time_start = time::now();

        match device.poll_events() {
            EventPumpAction::Quit => break 'running,
            EventPumpAction::Continue => {}
        }

        let old_rotation = mesh_cube.rotation;
        mesh_cube.set_rotation(Vector3::new(old_rotation.x + 0.01,
                                            old_rotation.y + 0.01,
                                            old_rotation.z));

        mesh_cube2.set_rotation(Vector3::new(old_rotation.x + 0.01,
                                             old_rotation.y + 0.01,
                                             old_rotation.z));

        device.clear(Color::RGB(0, 0, 128));
        device.render(&cam, vec![&mesh_cube, &mesh_cube2]);
        device.present();

        let time_end = time::now();
        let elapsed = (time_end - time_start).num_milliseconds();

        println!("frame = {:?}, elapsed = {}ms, fps = {}",
                 ticks,
                 elapsed,
                 1000.0 / elapsed as f32);
        ticks += 1;
    }
}
