use crate::math::*;
use crate::mesh::*;
use core::fmt;
use std::error::Error;
use std::fs;

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

impl Scene {
    pub fn load_from_file(path: &str) -> Result<Scene, Box<dyn Error>> {
        let file_content = fs::read_to_string(path)?.replace("\n", "");
        parse_scene_file(&file_content)?;
        Ok(Scene {
            camera: Camera::new(1, 1, 1.0, 1.0, 1.0),
            lights: vec![],
            meshes: vec![],
        })
    }
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
    pub node_name: String,
    pub expression: String,
}
impl Error for XMLParseError {}

impl fmt::Display for XMLParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Failed parsing node {} at expression {}",
            self.node_name, self.expression
        )
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct XMLNode {
    pub name: String,
    pub attributes: Vec<String>,
    pub data: Option<f64>,
    pub children: Vec<XMLNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum XMLToken {
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
        node_name: "tokenizer".to_string(),
        expression: "tokenization".to_string(),
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
    let err = XMLParseError {
        node_name: "unknown".to_string(),
        expression: "tag start".to_string(),
    };

    let Some(XMLToken::OpenBracket) = tokens.peek() else {
        tokens.restore_checkpoint(start_checkpoint);
        return Err(err);
    };
    tokens.consume();

    let Some(XMLToken::Name(tag_name)) = tokens.peek() else {
        tokens.restore_checkpoint(start_checkpoint);
        return Err(err);
    };
    tokens.consume();

    // (note: amoussa) this copy seems like it could be avoided but oh well
    node.name = tag_name.to_string();

    let Some(XMLToken::CloseBracket) = tokens.peek() else {
        tokens.restore_checkpoint(start_checkpoint);
        return Err(err);
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
    let err = XMLParseError {
        node_name: "unknown".to_string(),
        expression: "tag start and end".to_string(),
    };

    let Some(XMLToken::OpenBracket) = tokens.peek() else {
        tokens.restore_checkpoint(start_checkpoint);
        return Err(err);
    };
    tokens.consume();

    let Some(XMLToken::Name(tag_name)) = tokens.peek() else {
        tokens.restore_checkpoint(start_checkpoint);
        return Err(err);
    };
    tokens.consume();

    // (note: amoussa) this copy seems like it could be avoided but oh well
    node.name = tag_name.to_string();

    let Some(XMLToken::CloseSlashBracket) = tokens.peek() else {
        tokens.restore_checkpoint(start_checkpoint);
        return Err(err);
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
    let err = XMLParseError {
        node_name: node.name.clone(),
        expression: "tag end".to_string(),
    };

    let Some(XMLToken::OpenSlashBracket) = tokens.peek() else {
        tokens.restore_checkpoint(start_checkpoint);
        return Err(err);
    };
    tokens.consume();

    let Some(XMLToken::Name(tag_name)) = tokens.peek() else {
        tokens.restore_checkpoint(start_checkpoint);
        return Err(err);
    };
    tokens.consume();

    // make sure start and end tag match
    if *tag_name != node.name {
        tokens.restore_checkpoint(start_checkpoint);
        return Err(err);
    }

    let Some(XMLToken::CloseBracket) = tokens.peek() else {
        tokens.restore_checkpoint(start_checkpoint);
        return Err(err);
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
