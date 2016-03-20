#![feature(alloc_system)]
extern crate alloc_system;

extern crate sdl2;
extern crate cgmath;

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

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

fn main() {
    let mut device = Device::new("soft-renderer", WIDTH, HEIGHT);

    let mut ticks = 0;

    let cam = Camera::new();

    let mut mesh_cube = Mesh::new("cube",
                                  vec![Vector3::new(-1.0, 1.0, 1.0),
                                       Vector3::new(1.0, 1.0, 1.0),
                                       Vector3::new(-1.0, -1.0, 1.0),
                                       Vector3::new(-1.0, -1.0, -1.0),
                                       Vector3::new(-1.0, 1.0, -1.0),
                                       Vector3::new(1.0, 1.0, -1.0),
                                       Vector3::new(1.0, -1.0, 1.0),
                                       Vector3::new(1.0, -1.0, -1.0)]);

    //    let mut mesh_cube = Mesh::new("cube",
    //                                  vec![Vector3::new(-0.1, 0.1, 0.1),
    //                                       Vector3::new(0.1, 0.1, 0.1),
    //                                       Vector3::new(-0.1, -0.1, 0.1),
    //                                       Vector3::new(-0.1, -0.1, -0.1),
    //                                       Vector3::new(-0.1, 0.1, -0.1),
    //                                       Vector3::new(0.1, 0.1, -0.1),
    //                                       Vector3::new(0.1, -0.1, 0.1),
    //                                       Vector3::new(0.1, -0.1, -0.1)]);

        let mut mesh_cube2 = Mesh::new("cube",
                                       vec![Vector3::new(-0.05, 0.05, 0.05),
                                            Vector3::new(0.05, 0.05, 0.05),
                                            Vector3::new(-0.05, -0.05, 0.05),
                                            Vector3::new(-0.05, -0.05, -0.05),
                                            Vector3::new(-0.05, 0.05, -0.05),
                                            Vector3::new(0.05, 0.05, -0.05),
                                            Vector3::new(0.05, -0.05, 0.05),
                                            Vector3::new(0.05, -0.05, -0.05)]);


    mesh_cube.set_position(Vector3::zero());

    'running: loop {
        match device.poll_events() {
            EventPumpAction::Quit => break 'running,
            EventPumpAction::Continue => {}
        }

        device.clear(Color::RGB(0, 0, 128));

        let old_rotation = mesh_cube.rotation;
        mesh_cube.set_rotation(Vector3::new(old_rotation.x + 0.01,
                                            old_rotation.y + 0.01,
                                            old_rotation.z));

        mesh_cube2.set_rotation(Vector3::new(old_rotation.x + 0.01,
                                             old_rotation.y + 0.01,
                                             old_rotation.z));

        device.render(&cam, vec![&mesh_cube2]);

        device.present();

        println!("frame = {:?}", ticks);
        ticks += 1;
    }
}
