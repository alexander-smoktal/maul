use ast::expressions::*;

use super::utils::*;

#[test]
fn test_label() {
    assert_eq!(statements::parse_statement(&mut make_lexer(":: label ::")),
               Ok(Expression::Label("label".to_string())));
}

#[test]
fn test_goto() {
    assert_eq!(statements::parse_statement(&mut make_lexer("goto label")),
               Ok(Expression::Goto("label".to_string())));
}