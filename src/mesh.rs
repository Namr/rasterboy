use crate::math::*;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::error::Error;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Triangle {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

#[derive(Debug, Default, Clone)]
pub struct Mesh {
    pub verticies: Vec<Vector3>,
    pub face_indicies: Vec<Triangle>,
}

impl Mesh {
    pub fn from_obj_file(path: &Path) -> Result<Mesh, Box<dyn Error>> {
        let obj_file = File::open(path)?;
        let mut ret = Mesh::default();

        // read line by line, insert all verts into ret
        let obj_reader = BufReader::new(obj_file);
        for maybe_line in obj_reader.lines() {
            let line = maybe_line?;

            let split_line: Vec<&str> = line.split_whitespace().collect();
            if split_line.len() != 4 {
                continue;
            }

            match split_line[0] {
                "v" => {
                    let x = split_line[1].parse::<f32>()?;
                    let y = split_line[2].parse::<f32>()?;
                    let z = split_line[3].parse::<f32>()?;
                    ret.verticies.push(Vector3 { x, y, z });
                }
                "f" => {
                    let a = split_line[1].parse::<usize>()?;
                    let b = split_line[2].parse::<usize>()?;
                    let c = split_line[3].parse::<usize>()?;
                    ret.face_indicies.push(Triangle {
                        a: a - 1,
                        b: b - 1,
                        c: c - 1,
                    });
                }
                _ => continue,
            }
        }
        Ok(ret)
    }
}
