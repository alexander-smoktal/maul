use ast::expressions::*;
use ast::expressions::statements::*;

use super::utils::make_lexer;

#[test]
fn test_block() {
    assert_eq!(blocks::parse_block(&mut make_lexer("one = 8
                                                    return nil, false, true;"))
               , Ok(Expression::Expressions(
                   vec![
                       Box::new(Expression::Expressions(vec![Box::new(Expression::Assignment(
                           Box::new(Expression::Id(vec!["one".to_string()])),
                           Box::new(Expression::Number(8f64))))])),
                       Box::new(Expression::St(Statement::Return(Box::new(
                           Expression::Expressions(vec![
                               Box::new(Expression::Nil)
                                   , Box::new(Expression::Boolean(false))
                                   , Box::new(Expression::Boolean(true))])))))])))
}

