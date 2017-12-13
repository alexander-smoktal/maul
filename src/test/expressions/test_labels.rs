use ast::expressions::*;

use super::utils::*;

#[test]
fn test_label() {
    assert_eq!(statements::parse_statement(&mut make_lexer(":: label ::")),
               Ok(Box::new(labels::Label("label".to_string())) as Box<expression::Expression>));
}

#[test]
fn test_goto() {
    assert_eq!(statements::parse_statement(&mut make_lexer("goto label")),
               Ok(Box::new(labels::Goto("label".to_string())) as Box<expression::Expression>));
}