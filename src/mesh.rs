use crate::image::*;
use crate::math::*;
use core::fmt;
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
    pub a_normal: usize,
    pub b_normal: usize,
    pub c_normal: usize,
    pub a_texture: usize,
    pub b_texture: usize,
    pub c_texture: usize,
}

#[derive(Debug, Default, Clone)]
pub struct Mesh {
    pub verticies: Vec<Vector3>,
    pub face_indicies: Vec<Triangle>,
    pub vertex_normals: Vec<Vector3>,
    pub vertex_texture_coords: Vec<Vector3>,
    pub texture: Option<Image>,
}

#[derive(Debug)]
pub struct ParseObjError {}
impl Error for ParseObjError {}

impl fmt::Display for ParseObjError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Obj file did not match expected format")
    }
}

impl Mesh {
    pub fn from_obj_file(path: &Path) -> Result<Mesh, Box<dyn Error>> {
        let obj_file = File::open(path)?;
        let mut ret = Mesh::default();

        let mut triangle_to_faces: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut should_compute_normals = true;

        // read line by line, insert all verts into ret
        let obj_reader = BufReader::new(obj_file);
        for maybe_line in obj_reader.lines() {
            let line = maybe_line?;

            let split_line: Vec<&str> = line.split_whitespace().collect();

            match split_line[0] {
                "v" => {
                    let x = split_line[1].parse::<f32>()?;
                    let y = split_line[2].parse::<f32>()?;
                    let z = split_line[3].parse::<f32>()?;
                    ret.verticies.push(Vector3 { x, y, z });
                }
                "vn" => {
                    let x = split_line[1].parse::<f32>()?;
                    let y = split_line[2].parse::<f32>()?;
                    let z = split_line[3].parse::<f32>()?;
                    ret.vertex_normals.push(Vector3 { x, y, z }.normalized());
                }
                "vt" => {
                    let x = split_line[1].parse::<f32>()?;
                    let y = split_line[2].parse::<f32>()?;
                    // FIXME make vector2
                    ret.vertex_texture_coords.push(Vector3 { x, y, z: 0.0 });
                }
                "f" => {
                    ret.face_indicies
                        .push(parse_face(&line).ok_or(ParseObjError {})?);
                    let face_index = ret.face_indicies.len() - 1;
                    let face_ref: &Triangle = ret.face_indicies.last().unwrap();

                    // (note: amoussa) this is not great, but we say that if every
                    // single face has the same vertex index and normal index, then we should
                    // generate normals (since that output is what happens if there were no normals
                    // in the file). Ideally the parse_face function should just tell us if normals
                    // were present in the file though.
                    let normals_and_vert_idxs_are_the_same = face_ref.a == face_ref.a_normal
                        && face_ref.b == face_ref.b_normal
                        && face_ref.c == face_ref.c_normal;
                    should_compute_normals &= normals_and_vert_idxs_are_the_same;

                    if should_compute_normals {
                        // store for normal generation
                        for t in [face_ref.a, face_ref.b, face_ref.c] {
                            let triangle_index = t;
                            match triangle_to_faces.get_mut(&triangle_index) {
                                Some(face_list) => face_list.push(face_index),
                                _ => {
                                    drop(triangle_to_faces.insert(triangle_index, vec![face_index]))
                                }
                            }
                        }
                    }
                }
                "mtllib" => {
                    let prefix = match path.parent() {
                        Some(pre) => pre,
                        None => Path::new(""),
                    };
                    let mat_lib = prefix.join(split_line[1]);
                    ret.texture = Some(load_texture_from_material_lib(&mat_lib)?);
                }
                _ => continue,
            }
        }

        // compute normals if they are missing
        if should_compute_normals {
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
        }
        Ok(ret)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FaceParseState {
    Ready,
    Number,
    Slash,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum CurrentNumberType {
    Vert,
    TextureCoord,
    Normal,
}

fn increment_number_type(current_type: CurrentNumberType) -> CurrentNumberType {
    match current_type {
        CurrentNumberType::Vert => CurrentNumberType::TextureCoord,
        CurrentNumberType::TextureCoord => CurrentNumberType::Normal,
        CurrentNumberType::Normal => CurrentNumberType::Vert,
    }
}

fn push_number_into_face(
    face: &mut Triangle,
    idx: usize,
    num: usize,
    num_type: CurrentNumberType,
) -> Option<()> {
    match num_type {
        CurrentNumberType::Vert => match idx {
            0 => face.a = num - 1,
            1 => face.b = num - 1,
            2 => face.c = num - 1,
            _ => return None,
        },
        CurrentNumberType::Normal => match idx {
            0 => face.a_normal = num - 1,
            1 => face.b_normal = num - 1,
            2 => face.c_normal = num - 1,
            _ => return None,
        },
        CurrentNumberType::TextureCoord => match idx {
            0 => face.a_texture = num - 1,
            1 => face.b_texture = num - 1,
            2 => face.c_texture = num - 1,
            _ => return None,
        },
    }

    Some(())
}

fn parse_face(face_str: &str) -> Option<Triangle> {
    let mut state = FaceParseState::Ready;
    let mut num_type = CurrentNumberType::Vert;
    let mut vert_idx = 0;
    let mut tmp_num_str = "".to_string();
    let mut ret = Triangle::default();
    let mut seen_normals = false;

    for c in face_str.chars() {
        match state {
            FaceParseState::Ready => {
                if c.is_numeric() {
                    tmp_num_str.clear();
                    state = FaceParseState::Number;
                    tmp_num_str.push(c);
                } else if c.is_whitespace() || c == 'f' {
                    continue;
                } else {
                    return None;
                }
            }
            FaceParseState::Number => {
                if c.is_numeric() {
                    state = FaceParseState::Number;
                    tmp_num_str.push(c);
                } else if c == '/' {
                    push_number_into_face(
                        &mut ret,
                        vert_idx,
                        tmp_num_str.parse::<usize>().ok()?,
                        num_type,
                    );
                    num_type = increment_number_type(num_type);
                    state = FaceParseState::Slash;
                } else if c.is_whitespace() {
                    push_number_into_face(
                        &mut ret,
                        vert_idx,
                        tmp_num_str.parse::<usize>().ok()?,
                        num_type,
                    );
                    seen_normals |= num_type == CurrentNumberType::Normal;
                    num_type = CurrentNumberType::Vert;
                    state = FaceParseState::Ready;
                    vert_idx += 1;
                } else {
                    return None;
                }
            }
            FaceParseState::Slash => {
                if c.is_numeric() {
                    tmp_num_str.clear();
                    state = FaceParseState::Number;
                    tmp_num_str.push(c);
                } else if c == '/' {
                    num_type = increment_number_type(num_type);
                    state = FaceParseState::Ready;
                } else {
                    return None;
                }
            }
        }
    }

    if state == FaceParseState::Number && !tmp_num_str.is_empty() {
        push_number_into_face(
            &mut ret,
            vert_idx,
            tmp_num_str.parse::<usize>().ok()?,
            num_type,
        );
    }

    // if we didn't see normals insert the default indicies
    if !seen_normals {
        ret.a_normal = ret.a;
        ret.b_normal = ret.b;
        ret.c_normal = ret.c;
    }
    Some(ret)
}

fn load_texture_from_material_lib(mat_path: &Path) -> Result<Image, Box<dyn Error>> {
    // load file
    let file = File::open(mat_path)?;
    let reader = BufReader::new(file);

    for maybe_line in reader.lines() {
        let line = maybe_line?;
        let split_line: Vec<&str> = line.split_whitespace().collect();
        if !split_line.is_empty() && split_line[0] == "map_Kd" {
            let path = Path::new(split_line[1]);
            return Image::load_ppm(path);
        }
    }

    Err(Box::new(ParseObjError {}))
}

#[cfg(test)]
mod test {
    use crate::mesh::*;

    #[test]
    fn test_face_parse_vert_only() {
        let face_str = "f 1 2 3";
        let maybe_tri = parse_face(face_str);
        assert!(maybe_tri.is_some());

        let tri = maybe_tri.unwrap();
        assert_eq!(tri.a, 0);
        assert_eq!(tri.b, 1);
        assert_eq!(tri.c, 2);

        assert_eq!(tri.a_normal, 0);
        assert_eq!(tri.b_normal, 1);
        assert_eq!(tri.c_normal, 2);
    }

    #[test]
    fn test_face_parse_vert_normal() {
        let face_str = "f 1//5 2//7 3//8";
        let maybe_tri = parse_face(face_str);
        assert!(maybe_tri.is_some());

        let tri = maybe_tri.unwrap();
        assert_eq!(tri.a, 0);
        assert_eq!(tri.b, 1);
        assert_eq!(tri.c, 2);

        assert_eq!(tri.a_normal, 4);
        assert_eq!(tri.b_normal, 6);
        assert_eq!(tri.c_normal, 7);
    }

    #[test]
    fn test_face_parse_vert_texture() {
        let face_str = "f 1/5 2/72 3/8";
        let maybe_tri = parse_face(face_str);
        assert!(maybe_tri.is_some());

        let tri = maybe_tri.unwrap();
        assert_eq!(tri.a, 0);
        assert_eq!(tri.b, 1);
        assert_eq!(tri.c, 2);

        assert_eq!(tri.a_normal, 0);
        assert_eq!(tri.b_normal, 1);
        assert_eq!(tri.c_normal, 2);

        assert_eq!(tri.a_texture, 4);
        assert_eq!(tri.b_texture, 71);
        assert_eq!(tri.c_texture, 7);
    }

    #[test]
    fn test_face_parse_vert_texture_normal() {
        let face_str = "f 1/5/7 2/72/8 3/8/9";
        let maybe_tri = parse_face(face_str);
        assert!(maybe_tri.is_some());

        let tri = maybe_tri.unwrap();
        assert_eq!(tri.a, 0);
        assert_eq!(tri.b, 1);
        assert_eq!(tri.c, 2);

        assert_eq!(tri.a_texture, 4);
        assert_eq!(tri.b_texture, 71);
        assert_eq!(tri.c_texture, 7);

        assert_eq!(tri.a_normal, 6);
        assert_eq!(tri.b_normal, 7);
        assert_eq!(tri.c_normal, 8);
    }

    #[test]
    fn test_face_parse_invalid() {
        let face_str = "f 1///5/7 2/72/8 3/8/9";
        let maybe_tri = parse_face(face_str);
        assert!(maybe_tri.is_none());
    }
}
