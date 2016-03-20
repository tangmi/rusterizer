extern crate cgmath;

use cgmath::Vector;
use cgmath::Vector3;

use std::vec::Vec;

pub struct Mesh {
    pub name: &'static str,
    pub vertices: Vec<Vector3<f64>>,
    pub position: Vector3<f64>,
    pub rotation: Vector3<f64>, // todo: uh...
}

impl Mesh {
    pub fn new(name: &'static str, verts: Vec<Vector3<f64>>) -> Mesh {
        Mesh {
            name: name,
            vertices: verts,
            position: Vector3::zero(),
            rotation: Vector3::zero(),
        }
    }

    pub fn set_position(&mut self, new_pos: Vector3<f64>) {
        self.position = new_pos;
    }

    pub fn set_rotation(&mut self, new_rot: Vector3<f64>) {
        self.rotation = new_rot;
    }
}
