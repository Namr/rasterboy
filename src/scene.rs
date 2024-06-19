use crate::math::*;
use crate::mesh::*;
use crate::rasterizer::draw_mesh;
use core::fmt;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Default, Copy, Clone)]
pub struct Camera {
    pub near_plane: f32,
    pub far_plane: f32,
    pub canvas_width: i32,
    pub canvas_height: i32,
    pub view_mat: Mat4,
    pub projection_mat: Mat4,
}

// TODO: LookatCameraDefinition

#[derive(Debug, Default, Copy, Clone)]
pub struct Light {
    pub position: Vector3,
    pub color: Color,
    pub ambient_strength: f32,
}

#[derive(Debug, Default, Clone)]
pub struct Model {
    pub mesh: Mesh,
    pub transform: Mat4,
}

#[derive(Debug, Default, Clone)]
pub struct Scene {
    pub camera: Camera,
    pub models: Vec<Model>,
    pub lights: Vec<Light>,
}

#[derive(Debug)]
pub struct SceneLoadError {
    pub msg: String,
}
impl Error for SceneLoadError {}

impl fmt::Display for SceneLoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed Scene Loading with error {}", self.msg,)
    }
}

impl Scene {
    pub fn load_from_file(path: &str) -> Result<Scene, Box<dyn Error>> {
        let file_content = fs::read_to_string(path)?.replace("\n", "");
        let xml_node = parse_scene_file(&file_content)?;

        // FIXME: this needs to be loaded from the file
        const IMAGE_WIDTH: i32 = 1920;
        const IMAGE_HEIGHT: i32 = 1080;
        const NEAR: f32 = 0.1;
        const FAR: f32 = 100.0;
        let mut scene = Scene {
            camera: Camera::new(IMAGE_WIDTH, IMAGE_HEIGHT, 54_f32.to_radians(), NEAR, FAR),
            models: vec![],
            lights: vec![],
        };

        if xml_node.name != "file" {
            return Err(Box::new(SceneLoadError {
                msg: "XML file was malformed".to_string(),
            }));
        }
        if xml_node.children.len() != 1 {
            return Err(Box::new(SceneLoadError {
                msg: "No scene tag found".to_string(),
            }));
        }
        let scene_node = &xml_node.children[0];

        // look over scene node children for camera, lights, models
        for child_node in scene_node.children.iter() {
            match child_node.name.as_str() {
                "model" => scene.models.push(model_from_xml_node(child_node)?),
                "light" => scene.lights.push(light_from_xml_node(child_node)?),
                name => {
                    return Err(Box::new(SceneLoadError {
                        msg: format!("Unknown tag {} found", name),
                    }))
                }
            }
        }
        Ok(scene)
    }

    pub fn render(self, pixel_buffer: &mut [Color], depth_buffer: &mut [f32]) {
        for model in self.models.iter() {
            draw_mesh(
                &model.mesh,
                model.transform,
                &self.lights,
                self.camera,
                pixel_buffer,
                depth_buffer,
            );
        }
    }
}

fn model_from_xml_node(model_node: &XMLNode) -> Result<Model, Box<dyn Error>> {
    let mut model: Model = Default::default();
    model.transform = Mat4::identity();
    for model_property in model_node.children.iter() {
        // TODO: enforce exactly one of each property
        match model_property.name.as_str() {
            "mesh" => {
                if model_property.children.len() != 1 {
                    return Err(Box::new(SceneLoadError {
                        msg: "mesh tag did not specify a path".to_string(),
                    }));
                }
                model.mesh = Mesh::from_obj_file(Path::new(&model_property.children[0].name))?;
            }
            "rotation" => {
                if model_property.children.len() != 3 {
                    return Err(Box::new(SceneLoadError {
                        msg: "rotation tag did not specify three numbers (RPY)".to_string(),
                    }));
                }
                let r = model_property.children[0]
                    .data
                    .ok_or(Box::new(SceneLoadError {
                        msg: "rotation tag contained something other than a number".to_string(),
                    }))?;
                let p = model_property.children[1]
                    .data
                    .ok_or(Box::new(SceneLoadError {
                        msg: "rotation tag contained something other than a number".to_string(),
                    }))?;
                let y = model_property.children[2]
                    .data
                    .ok_or(Box::new(SceneLoadError {
                        msg: "rotation tag contained something other than a number".to_string(),
                    }))?;
                model.transform = model.transform * Mat4::euler_angles(r, p, y);
            }
            "position" => {
                if model_property.children.len() != 3 {
                    return Err(Box::new(SceneLoadError {
                        msg: "position tag did not specify three numbers (XYZ)".to_string(),
                    }));
                }
                let x = model_property.children[0]
                    .data
                    .ok_or(Box::new(SceneLoadError {
                        msg: "position tag contained something other than a number".to_string(),
                    }))?;
                let y = model_property.children[1]
                    .data
                    .ok_or(Box::new(SceneLoadError {
                        msg: "position tag contained something other than a number".to_string(),
                    }))?;
                let z = model_property.children[2]
                    .data
                    .ok_or(Box::new(SceneLoadError {
                        msg: "position tag contained something other than a number".to_string(),
                    }))?;
                model.transform = model.transform * Mat4::translation(x, y, z);
            }
            name => {
                return Err(Box::new(SceneLoadError {
                    msg: format!("model had an unknown property {}", name),
                }))
            }
        }
    }

    Ok(model)
}

fn light_from_xml_node(light_node: &XMLNode) -> Result<Light, Box<dyn Error>> {
    let mut light: Light = Default::default();

    for light_property in light_node.children.iter() {
        // TODO: enforce exactly one of each property
        match light_property.name.as_str() {
            "strength" => {
                if light_property.children.len() != 1 {
                    return Err(Box::new(SceneLoadError {
                        msg: "strength tag did not specify a single number".to_string(),
                    }));
                }
                light.ambient_strength =
                    light_property.children[0]
                        .data
                        .ok_or(Box::new(SceneLoadError {
                            msg: "strength tag contained something other than a number".to_string(),
                        }))?;
            }
            "color" => {
                if light_property.children.len() != 3 {
                    return Err(Box::new(SceneLoadError {
                        msg: "color tag did not specify three numbers (RGB)".to_string(),
                    }));
                }
                let r = light_property.children[0]
                    .data
                    .ok_or(Box::new(SceneLoadError {
                        msg: "color tag contained something other than a number".to_string(),
                    }))?;
                let g = light_property.children[1]
                    .data
                    .ok_or(Box::new(SceneLoadError {
                        msg: "color tag contained something other than a number".to_string(),
                    }))?;
                let b = light_property.children[2]
                    .data
                    .ok_or(Box::new(SceneLoadError {
                        msg: "color tag contained something other than a number".to_string(),
                    }))?;

                if r > 255.0 || r < 0.0 {
                    return Err(Box::new(SceneLoadError {
                        msg: "red value in color tag was not between 0 and 255".to_string(),
                    }));
                }

                if g > 255.0 || g < 0.0 {
                    return Err(Box::new(SceneLoadError {
                        msg: "green value in color tag was not between 0 and 255".to_string(),
                    }));
                }

                if b > 255.0 || b < 0.0 {
                    return Err(Box::new(SceneLoadError {
                        msg: "blue value in color tag was not between 0 and 255".to_string(),
                    }));
                }

                light.color.r = f32::floor(r) as u8;
                light.color.g = f32::floor(g) as u8;
                light.color.b = f32::floor(b) as u8;
            }
            "position" => {
                if light_property.children.len() != 3 {
                    return Err(Box::new(SceneLoadError {
                        msg: "position tag did not specify three numbers (XYZ)".to_string(),
                    }));
                }
                light.position.x =
                    light_property.children[0]
                        .data
                        .ok_or(Box::new(SceneLoadError {
                            msg: "position tag contained something other than a number".to_string(),
                        }))?;
                light.position.y =
                    light_property.children[1]
                        .data
                        .ok_or(Box::new(SceneLoadError {
                            msg: "position tag contained something other than a number".to_string(),
                        }))?;
                light.position.z =
                    light_property.children[2]
                        .data
                        .ok_or(Box::new(SceneLoadError {
                            msg: "position tag contained something other than a number".to_string(),
                        }))?;
            }
            name => {
                return Err(Box::new(SceneLoadError {
                    msg: format!("light had an unknown property {}", name),
                }))
            }
        }
    }

    Ok(light)
}

impl Camera {
    pub fn new(canvas_width: i32, canvas_height: i32, fov: f32, near: f32, far: f32) -> Camera {
        Camera {
            near_plane: near,
            far_plane: far,
            canvas_width,
            canvas_height,
            view_mat: Mat4::identity(),
            projection_mat: Mat4::perspective(
                canvas_width as f32 / canvas_height as f32,
                fov,
                near,
                far,
            ),
        }
    }
}

// (note: amoussa) oh no, I wrote my own lexer and parser for XML...

#[derive(Debug)]
pub struct XMLParseError {
    pub msg: String,
}
impl Error for XMLParseError {}

impl fmt::Display for XMLParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed XML Parsing with error {}", self.msg,)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct XMLNode {
    pub name: String,
    pub attributes: Vec<String>,
    pub data: Option<f32>,
    pub children: Vec<XMLNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum XMLToken {
    OpenBracket,
    CloseBracket,
    OpenSlashBracket,
    CloseSlashBracket,
    Equals,
    Number(f32),
    Name(String),
    Quote(String),
}

#[derive(Debug, Clone, PartialEq)]
enum RegexStates {
    Ready,
    StartBracket,
    Slash,
    InNumber,
    InName,
    InQuote,
}

pub struct TokenizedFile {
    pub tokens: Vec<XMLToken>,
    current_index: usize,
}

impl TokenizedFile {
    pub fn push(&mut self, token: XMLToken) {
        self.tokens.push(token)
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.len() == self.current_index
    }

    pub fn peek(&self) -> Option<XMLToken> {
        if self.current_index >= self.tokens.len() {
            None
        } else {
            Some(self.tokens[self.current_index].clone())
        }
    }

    pub fn consume(&mut self) {
        self.current_index += 1
    }

    // these two just save and restore the current index such that if matching errors out, we can
    // go back to the pre-error token state
    pub fn save_checkpoint(&self) -> usize {
        self.current_index
    }

    pub fn restore_checkpoint(&mut self, checkpoint: usize) {
        self.current_index = checkpoint
    }
}

pub fn parse_scene_file(raw_text: &str) -> Result<XMLNode, XMLParseError> {
    let mut tokenized_file = lex_scene_file(raw_text).ok_or(XMLParseError {
        msg: "unsupported character in file".to_string(),
    })?;

    let mut node = XMLNode {
        name: "file".to_string(),
        attributes: vec![],
        data: None,
        children: vec![],
    };

    match parse_xml_node(&mut tokenized_file, &mut node) {
        Ok(_) => Ok(node),
        Err(err) => Err(err),
    }
}

//  <tag> ::= <tag-start> <tag-content> <tag-end>
//          | <tag-start-and-end>
fn parse_xml_node(tokens: &mut TokenizedFile, node: &mut XMLNode) -> Result<(), XMLParseError> {
    // base case
    if tokens.is_empty() {
        return Ok(());
    }

    let start_checkpoint = tokens.save_checkpoint();

    // parse a child tag
    let mut child: XMLNode = XMLNode::default();

    if parse_tag_start(tokens, &mut child).is_err() {
        // if its a single tag terminate early
        match parse_tag_start_and_end(tokens, &mut child) {
            Err(start_end_err) => {
                tokens.restore_checkpoint(start_checkpoint);
                return Err(start_end_err);
            }
            Ok(_) => {
                node.children.push(child);
                return Ok(());
            }
        }
    }

    if let Err(content_err) = parse_tag_content(tokens, &mut child) {
        tokens.restore_checkpoint(start_checkpoint);
        return Err(content_err);
    }

    if let Err(end_err) = parse_tag_end(tokens, &mut child) {
        tokens.restore_checkpoint(start_checkpoint);
        return Err(end_err);
    }

    node.children.push(child);

    // recurse
    Ok(())
}

// <tag-start> ::= "<" <name> ">"
fn parse_tag_start(tokens: &mut TokenizedFile, node: &mut XMLNode) -> Result<(), XMLParseError> {
    let start_checkpoint = tokens.save_checkpoint();

    let Some(XMLToken::OpenBracket) = tokens.peek() else {
        tokens.restore_checkpoint(start_checkpoint);
        return Err(XMLParseError {
            msg: "tag did not start with open bracket".to_string(),
        });
    };
    tokens.consume();

    let Some(XMLToken::Name(tag_name)) = tokens.peek() else {
        tokens.restore_checkpoint(start_checkpoint);
        return Err(XMLParseError {
            msg: "tag does not contain a name inside brackets".to_string(),
        });
    };
    tokens.consume();

    // (note: amoussa) this copy seems like it could be avoided but oh well
    node.name = tag_name.to_string();

    let Some(XMLToken::CloseBracket) = tokens.peek() else {
        tokens.restore_checkpoint(start_checkpoint);
        return Err(XMLParseError {
            msg: format!("{} tag did not end with a close bracket", tag_name),
        });
    };
    tokens.consume();

    Ok(())
}

// <tag-start-and-end> ::= "<" <name> "/>"
fn parse_tag_start_and_end(
    tokens: &mut TokenizedFile,
    node: &mut XMLNode,
) -> Result<(), XMLParseError> {
    let start_checkpoint = tokens.save_checkpoint();

    let Some(XMLToken::OpenBracket) = tokens.peek() else {
        tokens.restore_checkpoint(start_checkpoint);
        return Err(XMLParseError {
            msg: "tag did not start with open bracket".to_string(),
        });
    };
    tokens.consume();

    let Some(XMLToken::Name(tag_name)) = tokens.peek() else {
        tokens.restore_checkpoint(start_checkpoint);
        return Err(XMLParseError {
            msg: "tag does not contain a name inside brackets".to_string(),
        });
    };
    tokens.consume();

    // (note: amoussa) this copy seems like it could be avoided but oh well
    node.name = tag_name.to_string();

    let Some(XMLToken::CloseSlashBracket) = tokens.peek() else {
        tokens.restore_checkpoint(start_checkpoint);
        return Err(XMLParseError {
            msg: format!(
                "{} tag did not end with a close (or close slash />) bracket",
                tag_name
            ),
        });
    };
    tokens.consume();

    Ok(())
}

// <tag-content> = <number> <tag-content> | <quote> <tag-content> | <tag> <tag-content> | ""
fn parse_tag_content(tokens: &mut TokenizedFile, node: &mut XMLNode) -> Result<(), XMLParseError> {
    if let Some(XMLToken::Number(num)) = tokens.peek() {
        node.children.push(XMLNode {
            name: String::default(),
            data: Some(num),
            children: Vec::default(),
            attributes: Vec::default(),
        });
        tokens.consume();
        return parse_tag_content(tokens, node);
    }

    if let Some(XMLToken::Quote(name)) = tokens.peek() {
        node.children.push(XMLNode {
            name,
            data: None,
            children: Vec::default(),
            attributes: Vec::default(),
        });
        tokens.consume();
        return parse_tag_content(tokens, node);
    }

    if let Some(XMLToken::OpenBracket) = tokens.peek() {
        let _ = parse_xml_node(tokens, node)?;
        return parse_tag_content(tokens, node);
    }

    // empty content is ok
    Ok(())
}

// <tag-end> ::= "</" <name> ">"
fn parse_tag_end(tokens: &mut TokenizedFile, node: &mut XMLNode) -> Result<(), XMLParseError> {
    let start_checkpoint = tokens.save_checkpoint();

    let Some(XMLToken::OpenSlashBracket) = tokens.peek() else {
        tokens.restore_checkpoint(start_checkpoint);
        return Err(XMLParseError {
            msg: "closing tag does not contain a name inside brackets".to_string(),
        });
    };
    tokens.consume();

    let Some(XMLToken::Name(tag_name)) = tokens.peek() else {
        tokens.restore_checkpoint(start_checkpoint);
        return Err(XMLParseError {
            msg: "closing tag does not contain a name inside brackets".to_string(),
        });
    };
    tokens.consume();

    // make sure start and end tag match
    if *tag_name != node.name {
        tokens.restore_checkpoint(start_checkpoint);
        return Err(XMLParseError {
            msg: "closing tag name does not match opening tag name".to_string(),
        });
    }

    let Some(XMLToken::CloseBracket) = tokens.peek() else {
        tokens.restore_checkpoint(start_checkpoint);
        return Err(XMLParseError {
            msg: "tag did not end with a close bracket".to_string(),
        });
    };
    tokens.consume();

    Ok(())
}

// StartBracket either ends as < or </
// Slash must match as />
// Numbers accumulate until they run out of digits
// Names accumulate until they run out of alphanumerics
// Quotes accumulate until they hit another "
pub fn lex_scene_file(raw_text: &str) -> Option<TokenizedFile> {
    lex_scene_file_recursively(
        raw_text,
        TokenizedFile {
            tokens: vec![],
            current_index: 0,
        },
        RegexStates::Ready,
        vec![],
    )
}

fn lex_scene_file_recursively(
    text: &str,
    mut tokens: TokenizedFile,
    mut state: RegexStates,
    mut accumulator: Vec<char>,
) -> Option<TokenizedFile> {
    if text.is_empty() {
        Some(tokens)
    } else {
        let c = text.chars().next()?;
        let mut remaining_text = text;
        match state {
            RegexStates::Ready => {
                if c == '<' {
                    remaining_text = &text[1..];
                    state = RegexStates::StartBracket;
                } else if c == '/' {
                    remaining_text = &text[1..];
                    state = RegexStates::Slash;
                } else if c == '>' {
                    remaining_text = &text[1..];
                    state = RegexStates::Ready;
                    tokens.push(XMLToken::CloseBracket);
                } else if c == '=' {
                    remaining_text = &text[1..];
                    state = RegexStates::Ready;
                    tokens.push(XMLToken::Equals);
                } else if c == '"' {
                    remaining_text = &text[1..];
                    state = RegexStates::InQuote;
                } else if c.is_ascii_digit() || c == '-' {
                    accumulator.push(c);
                    remaining_text = &text[1..];
                    state = RegexStates::InNumber;
                } else if c.is_ascii_alphabetic() {
                    accumulator.push(c);
                    remaining_text = &text[1..];
                    state = RegexStates::InName;
                } else if c.is_whitespace() {
                    // consume but no state update
                    remaining_text = &text[1..];
                } else {
                    return None;
                }
            }
            RegexStates::Slash => {
                if c == '>' {
                    remaining_text = &text[1..];
                    state = RegexStates::Ready;
                    tokens.push(XMLToken::CloseSlashBracket);
                } else if c.is_whitespace() {
                    // consume but no state update
                    remaining_text = &text[1..];
                } else {
                    return None;
                }
            }
            RegexStates::StartBracket => {
                state = RegexStates::Ready;
                if c == '/' {
                    remaining_text = &text[1..];
                    tokens.push(XMLToken::OpenSlashBracket);
                } else {
                    // we do not consume here
                    tokens.push(XMLToken::OpenBracket);
                }
            }
            RegexStates::InName => {
                if c.is_ascii_alphanumeric() {
                    accumulator.push(c);
                    remaining_text = &text[1..];
                } else {
                    tokens.push(XMLToken::Name(accumulator.iter().collect()));
                    accumulator.clear();
                    // we do not consume the character here
                    state = RegexStates::Ready;
                }
            }
            RegexStates::InNumber => {
                if c.is_ascii_digit() || c == '.' {
                    accumulator.push(c);
                    remaining_text = &text[1..];
                } else {
                    tokens.push(XMLToken::Number(
                        accumulator.iter().collect::<String>().parse().ok()?,
                    ));
                    accumulator.clear();
                    // we do not consume the character here
                    state = RegexStates::Ready;
                }
            }
            RegexStates::InQuote => {
                if c == '"' {
                    tokens.push(XMLToken::Quote(accumulator.iter().collect()));
                    accumulator.clear();
                    state = RegexStates::Ready;
                    remaining_text = &text[1..];
                } else {
                    accumulator.push(c);
                    remaining_text = &text[1..];
                }
            }
        }
        lex_scene_file_recursively(remaining_text, tokens, state, accumulator)
    }
}
