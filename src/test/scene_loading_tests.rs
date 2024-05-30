use crate::scene::*;

#[test]
fn test_xml_lex_unnested() {
    let example_tag = "<pog></pog>";
    let tokens = lex_scene_file(example_tag);

    let actual_tokens = vec![
        XMLTokens::OpenBracket,
        XMLTokens::Name("pog".to_string()),
        XMLTokens::CloseBracket,
        XMLTokens::OpenSlashBracket,
        XMLTokens::Name("pog".to_string()),
        XMLTokens::CloseBracket,
    ];

    assert!(tokens.is_some());
    assert_eq!(tokens.unwrap(), actual_tokens);

    let example_tag_with_whitespace = "  <pog>  </pog>  ";
    let tokens = lex_scene_file(example_tag_with_whitespace);

    assert!(tokens.is_some());
    assert_eq!(tokens.unwrap(), actual_tokens);
}

#[test]
fn test_xml_lex_nested() {
    let example_tag = "<header/> <pog> <mynum> 1.567 </mynum></pog>";
    let tokens = lex_scene_file(example_tag);

    let actual_tokens = vec![
        XMLTokens::OpenBracket,
        XMLTokens::Name("header".to_string()),
        XMLTokens::CloseSlashBracket,
        XMLTokens::OpenBracket,
        XMLTokens::Name("pog".to_string()),
        XMLTokens::CloseBracket,
        XMLTokens::OpenBracket,
        XMLTokens::Name("mynum".to_string()),
        XMLTokens::CloseBracket,
        XMLTokens::Number(1.567),
        XMLTokens::OpenSlashBracket,
        XMLTokens::Name("mynum".to_string()),
        XMLTokens::CloseBracket,
        XMLTokens::OpenSlashBracket,
        XMLTokens::Name("pog".to_string()),
        XMLTokens::CloseBracket,
    ];

    assert!(tokens.is_some());
    assert_eq!(tokens.unwrap(), actual_tokens);
}
