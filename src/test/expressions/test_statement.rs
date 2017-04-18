use ast::expressions::*;
use ast::expressions::statements::*;

use super::utils::make_lexer;

#[test]
fn test_simple_statement() {
    assert_eq!(parse_statement(&mut make_lexer("break;"))
               , Ok(Expression::St(Statement::Break)))
}

#[test]
fn test_return_statement() {
    assert_eq!(parse_return_statement(&mut make_lexer("return nil, false, true;"))
               , Ok(Expression::St(Statement::Return(Box::new(
                   Expression::Expressions(vec![
                       Box::new(Expression::Nil)
                           , Box::new(Expression::Boolean(false))
                           , Box::new(Expression::Boolean(true))]))))))
}
