extern crate cgmath;

use cgmath::Point3;

#[derive(Debug)]
pub struct Camera {
    pub position: Point3<f64>,
    pub target: Point3<f64>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: Point3::new(0.0, 0.0, 10.0),
            target: Point3::new(0.0, 0.0, 0.0),
        }
    }
}
