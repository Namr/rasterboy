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

#[test]
fn test_xml_parse_unnested() {
    let example_tag = "<pog></pog>";
    let maybe_node = parse_scene_file(example_tag);

    // file node
    assert!(maybe_node.is_some());
    let node = maybe_node.unwrap();
    assert_eq!(node.name, "file");
    assert!(node.attributes.is_empty());
    assert!(node.data.is_none());
    assert_eq!(node.children.len(), 1);

    // pog node
    let maybe_pog_node = node.children.get(0);
    assert!(maybe_pog_node.is_some());
    let pog_node = maybe_pog_node.unwrap();
    assert_eq!(pog_node.name, "pog");
    assert!(pog_node.attributes.is_empty());
    assert!(pog_node.data.is_none());
    assert!(pog_node.children.is_empty());
}
