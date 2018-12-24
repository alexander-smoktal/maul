use super::utils::parse_string;
use crate::ast::rules;

#[test]
fn test_label() {
    assert_eq!(
        parse_string(":: Label ::", rules::label),
        r#"[Single(Label(Id("Label")))]"#
    );
}

#[test]
fn test_goto() {
    assert_eq!(
        parse_string("goto Label", rules::stat),
        r#"[Single(Goto(Id("Label")))]"#
    );
}
