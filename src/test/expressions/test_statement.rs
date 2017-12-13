use ast::expressions::*;
use ast::expressions::statements::*;

use super::utils::make_lexer;

#[test]
fn test_simple_statement() {
    assert_eq!(parse_statement(&mut make_lexer("break;"))
               , Ok(Box::new(Statement::Break) as Box<expression::Expression>))
}

#[test]
fn test_return_statement() {
    assert_eq!(parse_return_statement(&mut make_lexer("return nil, false, true;"))
               , Ok(Box::new(Statement::Return(Box::new(
                   util::Expressions(vec![
                       Box::new(primitives::Nil)
                           , Box::new(primitives::Boolean(false))
                           , Box::new(primitives::Boolean(true))])))) as Box<expression::Expression>))
}
