use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Result;

use std::f64;

use cgmath::Vector3;

use mesh::Mesh;
use mesh::Face;

pub fn import_file(filename: &str) -> Result<Mesh> {
    let f = try!(File::open(filename));
    let file = BufReader::new(f);
    let mesh = read_obj(filename, file.lines());
    Ok(mesh)
}


fn read_obj<I>(mesh_name: &str, file: I) -> Mesh where I: Iterator<Item = Result<String>> {
    let mut verts = Vec::new();
    let mut faces = Vec::new();

    for o_line in file {
        let line = o_line.unwrap();

        match parse_line(line) {
            Some(ObjLine::V(vertex)) => verts.push(vertex),
            Some(ObjLine::F(face)) => faces.push(face),
            _ => {},
        }
    }

    Mesh::new(mesh_name, verts, faces)
}

enum ObjLine {
    V(Vector3<f64>),
    F(Face),
}

fn parse_line(line: String) -> Option<ObjLine> {
    let mut iter = line.split_whitespace();

    let line_type = iter.next();

    match line_type {
        Some("v") => {
            let values : Vec<f64> = iter.filter_map(|s| s.parse().ok()).collect();

            if values.len() == 3 {
                let vertex = Vector3::new(values[0], values[1], values[2]);
                // println!("v {:?}", vertex);
                Some(ObjLine::V(vertex))
            } else {
                None
            }
        },
        Some("f") => {
            let indices : Vec<usize> = iter.filter_map(|s| {
                // is there a better way to do this?
                let mut indices = s.split('/');
                match indices.next() {
                    Some(s) => s.parse().ok(),
                    None => None,
                }
            }).collect();

            // TODO: 1-index, add texture coordinates, normals
            if indices.len() == 3 {
                let face = Face::new(indices[0] - 1, indices[1] - 1, indices[2] - 1);
                // println!("f {:?}", face);
                Some(ObjLine::F(face))
            } else {
                None
            }
        },
        _ => None,
    }
}
