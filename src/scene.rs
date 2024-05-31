use crate::math::*;
use crate::mesh::*;

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
pub struct Scene {
    pub camera: Camera,
    pub meshes: Vec<Mesh>,
    pub lights: Vec<Light>,
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

#[derive(Debug, Clone, PartialEq)]
pub enum XMLTokens {
    OpenBracket,
    CloseBracket,
    OpenSlashBracket,
    CloseSlashBracket,
    Equals,
    Number(f64),
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

// StartBracket either ends as < or </
// Slash must match as />
// Numbers accumulate until they run out of digits
// Names accumulate until they run out of alphanumerics
// Quotes accumulate until they hit another "
pub fn lex_scene_file(raw_text: &str) -> Option<Vec<XMLTokens>> {
    lex_scene_file_recursively(raw_text, vec![], RegexStates::Ready, vec![])
}

fn lex_scene_file_recursively(
    text: &str,
    mut tokens: Vec<XMLTokens>,
    mut state: RegexStates,
    mut accumulator: Vec<char>,
) -> Option<Vec<XMLTokens>> {
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
                    tokens.push(XMLTokens::CloseBracket);
                } else if c == '=' {
                    remaining_text = &text[1..];
                    state = RegexStates::Ready;
                    tokens.push(XMLTokens::Equals);
                } else if c == '"' {
                    remaining_text = &text[1..];
                    state = RegexStates::InQuote;
                } else if c.is_ascii_digit() {
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
                    tokens.push(XMLTokens::CloseSlashBracket);
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
                    tokens.push(XMLTokens::OpenSlashBracket);
                } else {
                    // we do not consume here
                    tokens.push(XMLTokens::OpenBracket);
                }
            }
            RegexStates::InName => {
                if c.is_ascii_alphanumeric() {
                    accumulator.push(c);
                    remaining_text = &text[1..];
                } else {
                    tokens.push(XMLTokens::Name(accumulator.iter().collect()));
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
                    tokens.push(XMLTokens::Number(
                        accumulator.iter().collect::<String>().parse().ok()?,
                    ));
                    accumulator.clear();
                    // we do not consume the character here
                    state = RegexStates::Ready;
                }
            }
            RegexStates::InQuote => {
                if c == '"' {
                    tokens.push(XMLTokens::Quote(accumulator.iter().collect()));
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
