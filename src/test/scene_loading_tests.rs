use crate::scene::*;

#[test]
fn test_xml_lex_unnested() {
    let example_tag = "<pog></pog>";
    let tokens = lex_scene_file(example_tag);

    let actual_tokens = vec![
        XMLToken::OpenBracket,
        XMLToken::Name("pog".to_string()),
        XMLToken::CloseBracket,
        XMLToken::OpenSlashBracket,
        XMLToken::Name("pog".to_string()),
        XMLToken::CloseBracket,
    ];

    assert!(tokens.is_some());
    assert_eq!(tokens.unwrap().tokens, actual_tokens);

    let example_tag_with_whitespace = "  <pog>  </pog>  ";
    let tokens = lex_scene_file(example_tag_with_whitespace);

    assert!(tokens.is_some());
    assert_eq!(tokens.unwrap().tokens, actual_tokens);
}

#[test]
fn test_xml_lex_nested() {
    let example_tag = "<header/> <pog class=\"humongus34\"> <mynum> 1.567 5 7.009 </mynum></pog>";
    let tokens = lex_scene_file(example_tag);

    let actual_tokens = vec![
        XMLToken::OpenBracket,
        XMLToken::Name("header".to_string()),
        XMLToken::CloseSlashBracket,
        XMLToken::OpenBracket,
        XMLToken::Name("pog".to_string()),
        XMLToken::Name("class".to_string()),
        XMLToken::Equals,
        XMLToken::Quote("humongus34".to_string()),
        XMLToken::CloseBracket,
        XMLToken::OpenBracket,
        XMLToken::Name("mynum".to_string()),
        XMLToken::CloseBracket,
        XMLToken::Number(1.567),
        XMLToken::Number(5.0),
        XMLToken::Number(7.009),
        XMLToken::OpenSlashBracket,
        XMLToken::Name("mynum".to_string()),
        XMLToken::CloseBracket,
        XMLToken::OpenSlashBracket,
        XMLToken::Name("pog".to_string()),
        XMLToken::CloseBracket,
    ];

    assert!(tokens.is_some());
    assert_eq!(tokens.unwrap().tokens, actual_tokens);
}

fn test_for_parent_tag(maybe_node: Option<&XMLNode>, name: &str, num_children: usize) {
    assert!(maybe_node.is_some());
    let node = maybe_node.unwrap();
    assert_eq!(node.name, name);
    assert!(node.attributes.is_empty());
    assert!(node.data.is_none());
    assert_eq!(node.children.len(), num_children);
}

fn test_for_childless_tag(maybe_node: Option<&XMLNode>, name: &str) {
    test_for_parent_tag(maybe_node, name, 0)
}

fn test_for_num(maybe_node: Option<&XMLNode>, number: f32) {
    assert!(maybe_node.is_some());
    let node = maybe_node.unwrap();
    assert_eq!(node.name, String::default());
    assert!(node.attributes.is_empty());
    assert!(node.data.is_some());
    assert_eq!(node.data.unwrap(), number);
    assert!(node.children.is_empty());
}

fn test_for_name(maybe_node: Option<&XMLNode>, name: &str) {
    assert!(maybe_node.is_some());
    let node = maybe_node.unwrap();
    assert_eq!(node.name, name);
    assert!(node.attributes.is_empty());
    assert!(node.data.is_none());
    assert!(node.children.is_empty());
}

#[test]
fn test_xml_parse_unnested() {
    let example_tag = "<pog></pog>";
    let maybe_node = parse_scene_file(example_tag);

    // file node
    assert!(maybe_node.is_ok());
    let node = maybe_node.unwrap();
    assert_eq!(node.name, "file");
    assert!(node.attributes.is_empty());
    assert!(node.data.is_none());
    assert_eq!(node.children.len(), 1);

    // pog node
    test_for_childless_tag(node.children.get(0), "pog");
}

#[test]
fn test_xml_parse_nested() {
    let example_tag = "
<scene>
  <mesh/>
  <light>
    1 2 3
  </light>
  <placeholder>
  \"some_names\"
  </placeholder>
  8
</scene>";

    let maybe_node = parse_scene_file(example_tag);

    // file node
    assert!(maybe_node.is_ok());
    let node = maybe_node.unwrap();
    assert_eq!(node.name, "file");
    assert!(node.attributes.is_empty());
    assert!(node.data.is_none());
    assert_eq!(node.children.len(), 1);

    let maybe_scene = node.children.get(0);
    test_for_parent_tag(maybe_scene, "scene", 4);

    test_for_childless_tag(maybe_scene.unwrap().children.get(0), "mesh");

    let maybe_light = maybe_scene.unwrap().children.get(1);
    test_for_parent_tag(maybe_light, "light", 3);

    test_for_num(maybe_light.unwrap().children.get(0), 1.0);
    test_for_num(maybe_light.unwrap().children.get(1), 2.0);
    test_for_num(maybe_light.unwrap().children.get(2), 3.0);

    let maybe_placeholder = maybe_scene.unwrap().children.get(2);
    test_for_parent_tag(maybe_placeholder, "placeholder", 1);
    test_for_name(maybe_placeholder.unwrap().children.get(0), "some_names");

    test_for_num(maybe_scene.unwrap().children.get(3), 8.0);
}

#[test]
fn test_xml_parse_no_end_tag() {
    let example_tag = "<pog>";
    let maybe_node = parse_scene_file(example_tag);

    let Err(parse_error) = maybe_node else {
        assert!(false);
        return;
    };
    assert!(!parse_error.msg.is_empty());
}

#[test]
fn test_xml_parse_end_tag_wrong_name() {
    let example_tag = "<pog> <dog/>";
    let maybe_node = parse_scene_file(example_tag);

    let Err(parse_error) = maybe_node else {
        assert!(false);
        return;
    };
    assert!(!parse_error.msg.is_empty());
}

#[test]
fn test_xml_parse_nested_no_close() {
    let example_tag = "<pog> <cool> <dool> <pog/>";
    let maybe_node = parse_scene_file(example_tag);

    let Err(parse_error) = maybe_node else {
        assert!(false);
        return;
    };
    assert!(!parse_error.msg.is_empty());
}

#[test]
fn test_xml_parse_has_garbage_input() {
    let example_tag = "
<scene>
  <mesh/>
  <light>
    1 2 3
  </light>
  <placeholder>
  egjeoig
  \"some_names\"
  </placeholder>
  8
</scene>";

    let maybe_node = parse_scene_file(example_tag);

    // file node
    assert!(maybe_node.is_err());
    let error = maybe_node.err().unwrap();
    assert!(!error.msg.is_empty());
}
