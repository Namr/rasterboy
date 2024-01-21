use crate::math::*;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

#[derive(Debug, Default)]
pub struct Triangle {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

#[derive(Debug, Default)]
pub struct Mesh {
    pub verticies: Vec<Vector3>,
    pub face_indicies: Vec<Triangle>,
}

impl Mesh {
    pub fn from_obj_file(path: &Path) -> Mesh {
        let path_display = path.display();
        let obj_file = match File::open(&path) {
            Err(why) => panic!("Couldn't open obj file {path_display}: {why}"),
            Ok(file) => file,
        };

        let mut ret = Mesh::default();

        // read line by line, insert all verts into ret
        let obj_reader = BufReader::new(obj_file);
        for maybe_line in obj_reader.lines() {
            let line = match maybe_line {
                Err(why) => panic!("Couldn't read line from obj file {path_display}: {why}"),
                Ok(line) => line,
            };

            let split_line: Vec<&str> = line.split_whitespace().collect();
            if split_line.len() != 4 {
                continue;
            }

            let parse_panic_msg = format!(
                "could not parse numbers from following line {line} in obj file {path_display}"
            );
            match *split_line.get(0).unwrap() {
                "v" => {
                    let x = split_line
                        .get(1)
                        .unwrap()
                        .parse::<f32>()
                        .expect(&parse_panic_msg);
                    let y = split_line
                        .get(2)
                        .unwrap()
                        .parse::<f32>()
                        .expect(&parse_panic_msg);
                    let z = split_line
                        .get(3)
                        .unwrap()
                        .parse::<f32>()
                        .expect(&parse_panic_msg);
                    ret.verticies.push(Vector3 { x, y, z });
                }
                "f" => {
                    let a = split_line
                        .get(1)
                        .unwrap()
                        .parse::<usize>()
                        .expect(&parse_panic_msg);
                    let b = split_line
                        .get(2)
                        .unwrap()
                        .parse::<usize>()
                        .expect(&parse_panic_msg);
                    let c = split_line
                        .get(3)
                        .unwrap()
                        .parse::<usize>()
                        .expect(&parse_panic_msg);
                    ret.face_indicies.push(Triangle {
                        a: a - 1,
                        b: b - 1,
                        c: c - 1,
                    });
                }
                _ => continue,
            }
        }

        ret
    }
}
