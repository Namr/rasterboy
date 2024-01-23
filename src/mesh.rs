use crate::math::*;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

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
    pub vertex_normals: Vec<Vector3>,
}

impl Mesh {
    pub fn from_obj_file(path: &Path) -> Result<Mesh, Box<dyn Error>> {
        let obj_file = File::open(path)?;
        let mut ret = Mesh::default();

        let mut triangle_to_faces: HashMap<usize, Vec<usize>> = HashMap::new();

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

                    let face_index = ret.face_indicies.len() - 1;

                    for t in [a, b, c] {
                        let triangle_index = t - 1;
                        match triangle_to_faces.get_mut(&triangle_index) {
                            Some(face_list) => face_list.push(face_index),
                            _ => drop(triangle_to_faces.insert(triangle_index, vec![face_index])),
                        }
                    }
                }
                _ => continue,
            }
        }

        // TODO: read in normals and only do this step if its actually needed
        // compute normals
        ret.vertex_normals = vec![Vector3::default(); ret.verticies.len()];
        for (triangle_idx, face_idx_list) in triangle_to_faces.into_iter() {
            // compute, sum, and then normalize the normals of every face that this vertex
            // contributes to
            ret.vertex_normals[triangle_idx] = face_idx_list
                .into_iter()
                .map(|face_idx| {
                    let v0 = ret.verticies[ret.face_indicies[face_idx].a];
                    let v1 = ret.verticies[ret.face_indicies[face_idx].b];
                    let v2 = ret.verticies[ret.face_indicies[face_idx].c];
                    Vector3::cross(v2 - v0, v1 - v0).normalized()
                })
                .fold(Vector3::default(), |acc, norm| acc + norm)
                .normalized();
        }
        Ok(ret)
    }
}
