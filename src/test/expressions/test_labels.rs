use super::utils::parse_string;
use ast::rules;

#[test]
fn test_label() {
    assert_eq!(parse_string(":: Label ::", rules::label), r#"[Single(Label(Id("Label")))]"#);
}

/*#[test]
fn test_goto() {
    assert_eq!(
        statements::parse_statement(&mut make_lexer("goto label")),
        Ok(exp!(labels::Goto("label".to_string())))
    );
}*/
