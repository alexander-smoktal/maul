use ast::expressions::*;
use ast::expressions::statements::*;

use super::utils::make_lexer;

#[test]
fn test_simple_statement() {
    assert_eq!(parse_statement(&mut make_lexer("break;"))
               , Ok(exp!(Statement::Break)))
}

#[test]
fn test_return_statement() {
    assert_eq!(parse_return_statement(&mut make_lexer("return nil, false, true;"))
               , Ok(exp!(Statement::Return(exp!(
                   util::Expressions(vec![
                       exp!(primitives::Nil)
                           , exp!(primitives::Boolean(false))
                           , exp!(primitives::Boolean(true))]))))))
}
