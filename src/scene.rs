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

#[derive(Debug, Clone, PartialEq, Default)]
struct XMLNode {
    name: String,
    attributes: Vec<String>,
    data: Option<f64>,
    children: Vec<XMLNode>,
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

struct TokenizedFile {
    tokens: Vec<XMLToken>,
    current_index: usize,
}

impl TokenizedFile {
    pub fn push(&mut self, token: XMLToken) {
        self.tokens.push(token)
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    pub fn peek(&self) -> Option<XMLToken> {
        if self.current_index >= self.tokens.len() || self.current_index < 0 {
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

pub fn parse_scene_file(raw_text: &str) -> Option<XMLNode> {
    let mut tokenized_file = lex_scene_file(raw_text)?;
    let node = XMLNode {
        name: "file".to_string(),
        attributes: vec![],
        data: None,
        children: vec![],
    };
    parse_xml_node(&mut tokenized_file, node)
}

//  <tag> ::= <tag-start> <tag-content> <tag-end>
//          | <tag-start-and-end>
fn parse_xml_node(tokens: &mut TokenizedFile, mut node: XMLNode) -> Option<XMLNode> {
    // base case
    if tokens.is_empty() {
        return Some(node);
    }

    let start_checkpoint = tokens.save_checkpoint();

    // parse a child tag
    let mut child: XMLNode = XMLNode::default();
    let Some(_) = parse_tag_start(tokens, &mut child) else {
        // if its a single tag terminate early
        if let Some(_) = parse_tag_start_and_end(tokens, &mut child) {
            return Some(node);
        } else {
            tokens.restore_checkpoint(start_checkpoint);
            return None;
        }
    };

    let Some(_) = parse_tag_content(tokens, &mut child) else {
        tokens.restore_checkpoint(start_checkpoint);
        return None;
    };

    let Some(_) = parse_tag_end(tokens, &mut child) else {
        tokens.restore_checkpoint(start_checkpoint);
        return None;
    };

    node.children.push(child);

    // recurse
    parse_xml_node(tokens, node)
}

// <tag-start> ::= "<" <name> ">"
fn parse_tag_start(tokens: &mut TokenizedFile, node: &mut XMLNode) -> Option<()> {
    let start_checkpoint = tokens.save_checkpoint();

    let Some(XMLToken::OpenBracket) = tokens.peek() else {
        tokens.restore_checkpoint(start_checkpoint);
        return None;
    };
    tokens.consume();

    let Some(XMLToken::Name(tag_name)) = tokens.peek() else {
        tokens.restore_checkpoint(start_checkpoint);
        return None;
    };
    tokens.consume();

    // (note: amoussa) this copy seems like it could be avoided but oh well
    node.name = tag_name.to_string();

    let Some(XMLToken::CloseBracket) = tokens.peek() else {
        tokens.restore_checkpoint(start_checkpoint);
        return None;
    };
    tokens.consume();

    Some(())
}

// <tag-start-and-end> ::= "<" <name> "/>"
fn parse_tag_start_and_end(tokens: &mut TokenizedFile, node: &mut XMLNode) -> Option<()> {
    let start_checkpoint = tokens.save_checkpoint();

    let Some(XMLToken::OpenBracket) = tokens.peek() else {
        tokens.restore_checkpoint(start_checkpoint);
        return None;
    };
    tokens.consume();

    let Some(XMLToken::Name(tag_name)) = tokens.peek() else {
        tokens.restore_checkpoint(start_checkpoint);
        return None;
    };
    tokens.consume();

    // (note: amoussa) this copy seems like it could be avoided but oh well
    node.name = tag_name.to_string();

    let Some(XMLToken::CloseSlashBracket) = tokens.peek() else {
        tokens.restore_checkpoint(start_checkpoint);
        return None;
    };
    tokens.consume();

    Some(())
}

// <tag-content> = <number> <tag-content> | <name> <tag-content> | <tag> | ""
fn parse_tag_content(tokens: &mut TokenizedFile, node: &mut XMLNode) -> Option<()> {
    if let Some(XMLToken::Number(num)) = tokens.peek() {
        node.children.push(XMLNode {
            name: String::default(),
            data: Some(*num),
            children: Vec::default(),
            attributes: Vec::default(),
        });
        tokens.consume();
        return parse_tag_content(tokens, node);
    }
    todo!();
    Some(())
}

// <tag-end> ::= "</" <name> ">"
fn parse_tag_end(tokens: &mut TokenizedFile, node: &mut XMLNode) -> Option<()> {
    let start_checkpoint = tokens.save_checkpoint();

    let Some(XMLToken::OpenSlashBracket) = tokens.peek() else {
        tokens.restore_checkpoint(start_checkpoint);
        return None;
    };
    tokens.consume();

    let Some(XMLToken::Name(tag_name)) = tokens.peek() else {
        tokens.restore_checkpoint(start_checkpoint);
        return None;
    };
    tokens.consume();

    // make sure start and end tag match
    if *tag_name != node.name {
        tokens.restore_checkpoint(start_checkpoint);
        return None;
    }

    let Some(XMLToken::CloseBracket) = tokens.peek() else {
        tokens.restore_checkpoint(start_checkpoint);
        return None;
    };
    tokens.consume();

    Some(())
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
