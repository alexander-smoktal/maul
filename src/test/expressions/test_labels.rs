use ast::expressions::*;

use super::utils::*;

#[test]
fn test_label() {
    assert_eq!(
        statements::parse_statement(&mut make_lexer(":: label ::")),
        Ok(exp!(labels::Label("label".to_string())))
    );
}

#[test]
fn test_goto() {
    assert_eq!(
        statements::parse_statement(&mut make_lexer("goto label")),
        Ok(exp!(labels::Goto("label".to_string())))
    );
}
