use ast::expressions::*;
use ast::expressions::statements::*;

use super::utils::make_lexer;

#[test]
fn test_block() {
    assert_eq!(blocks::parse_block(&mut make_lexer("one = 8
                                                    return nil, false, true;"))
               , Ok(Box::new(util::Expressions(
                   vec![
                       Box::new(util::Expressions(vec![Box::new(variables::Assignment(
                           Box::new(variables::Id(vec!["one".to_string()])),
                           Box::new(primitives::Number(8f64))))])),
                       Box::new(Statement::Return(Box::new(
                           util::Expressions(vec![
                               Box::new(primitives::Nil)
                                   , Box::new(primitives::Boolean(false))
                                   , Box::new(primitives::Boolean(true))]))))])) as Box<expression::Expression>))
}

#[test]
fn test_do_block() {
    assert_eq!(blocks::parse_do_block(&mut make_lexer("do one = 8 end"))
               , Ok(Box::new(blocks::DoBlock(
                   Box::new(
                       util::Expressions(
                        vec![
                            Box::new(util::Expressions(vec![Box::new(variables::Assignment(
                                Box::new(variables::Id(vec!["one".to_string()])),
                                Box::new(primitives::Number(8f64))))]))])))) as Box<expression::Expression>))
}

