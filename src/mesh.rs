extern crate cgmath;

use cgmath::Vector;
use cgmath::Vector3;

use std::vec::Vec;

// TODO(tang): should these fields have accessors?
#[derive(Debug)]
pub struct Mesh {
    pub name: String,
    pub vertices: Vec<Vector3<f64>>,
    pub faces: Vec<Face>,
    pub position: Vector3<f64>,
    pub rotation: Vector3<f64>, // TODO: consider using quaternions here
}

impl Mesh {
    pub fn new(name: &str, verts: Vec<Vector3<f64>>, faces: Vec<Face>) -> Mesh {
        Mesh {
            name: name.to_owned(),
            vertices: verts,
            faces: faces,
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

#[derive(Debug)]
pub struct Face {
    pub a: usize,
    pub b: usize,
    pub c: usize,
    // TODO: add texture coordinates
    // TODO: add normals
}

impl Face {
    pub fn new(a: usize, b: usize, c: usize) -> Face {
        Face { a: a, b: b, c: c }
    }
}
