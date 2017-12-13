use ast::lexer::tokens;
use ast::expressions::*;
use ast::expressions::statements::*;

use super::utils::*;

#[test]
fn test_exp_terminals() {
    assert_eq!(parse_exp(&mut make_lexer("nil")), Ok(Box::new(primitives::Nil) as Box<expression::Expression>));
    assert_eq!(parse_exp(&mut make_lexer("false")), Ok(Box::new(primitives::Boolean(false)) as Box<expression::Expression>));
    assert_eq!(parse_exp(&mut make_lexer("true")), Ok(Box::new(primitives::Boolean(true)) as Box<expression::Expression>));
    assert_eq!(parse_exp(&mut make_lexer("...")), Ok(Box::new(Statement::Ellipsis) as Box<expression::Expression>));
    assert_eq!(parse_exp(&mut make_lexer("42.42")), Ok(Box::new(primitives::Number(42.42f64)) as Box<expression::Expression>));
    assert_eq!(parse_exp(&mut make_lexer(r#""Hello""#)), Ok(Box::new(primitives::String("Hello".to_string())) as Box<expression::Expression>));
}

#[test]
fn test_exp_binop() {
    assert_eq!(parse_exp(&mut make_lexer("1 + 3")), Ok(Box::new(operators::Binop(tokens::Keyword::PLUS
                                                                         , Box::new(primitives::Number(1f64))
                                                                         , Box::new(primitives::Number(3f64)))) as Box<expression::Expression>));

    assert_eq!(parse_exp(&mut make_lexer("1.1 ~= 7")), Ok(Box::new(operators::Binop(tokens::Keyword::NEQ
                                                                            , Box::new(primitives::Number(1.1f64))
                                                                            , Box::new(primitives::Number(7f64)))) as Box<expression::Expression>));
}

#[test]
fn test_exp_unop() {
    assert_eq!(parse_exp(&mut make_lexer("-3")), Ok(Box::new(operators::Unop(tokens::Keyword::MINUS
                                                                     , Box::new(primitives::Number(3f64)))) as Box<expression::Expression>));

    assert_eq!(parse_exp(&mut make_lexer("#7")), Ok(Box::new(operators::Unop(tokens::Keyword::HASH
                                                                            , Box::new(primitives::Number(7f64)))) as Box<expression::Expression>));
}

#[test]
fn test_exp_prefix() {
    assert_eq!(parse_exp(&mut make_lexer("Hello.world")), Ok(Box::new(tables::Indexing {
        object: Box::new(variables::Id(vec!["Hello".to_owned()])),
        index: Box::new(primitives::String("world".to_owned())),
    }) as Box<expression::Expression>))
}

#[test]
fn test_exp_functiondef() {
    assert_eq!(parse_exp(&mut make_lexer("function f () break end")), Ok(make_assignment(vec!["f"],
                                                                                          Box::new(function::Function {
                                                                                              params: vec![],
                                                                                              body: Box::new(util::Expressions(vec![Box::new(Statement::Break)]))
                                                                                          }))))
}

#[test]
fn test_exp_table_constructor() {
    assert_eq!(parse_exp(&mut make_lexer("{ [true] = false }")), Ok(Box::new(tables::TableConstructor(vec![Box::new(
        variables::Assignment(
             Box::new(primitives::Boolean(true))
                , Box::new(primitives::Boolean(false))
        ))])) as Box<expression::Expression>));
}



