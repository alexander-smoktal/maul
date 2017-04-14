use ast::lexer::tokens;
use ast::expressions::*;

use super::utils::*;

#[test]
fn test_exp_terminals() {
    assert_eq!(parse_exp(&mut make_lexer("nil")), Ok(Expression::Nil));
    assert_eq!(parse_exp(&mut make_lexer("false")), Ok(Expression::Boolean(false)));
    assert_eq!(parse_exp(&mut make_lexer("true")), Ok(Expression::Boolean(true)));
    assert_eq!(parse_exp(&mut make_lexer("...")), Ok(Expression::St(statements::Statement::Ellipsis)));
    assert_eq!(parse_exp(&mut make_lexer("42.42")), Ok(Expression::Number(42.42f64)));
    assert_eq!(parse_exp(&mut make_lexer(r#""Hello""#)), Ok(Expression::String("Hello".to_string())));
}

#[test]
fn test_exp_binop() {
    assert_eq!(parse_exp(&mut make_lexer("1 + 3")), Ok(Expression::Binop(tokens::Keyword::PLUS
                                                                         , Box::new(Expression::Number(1f64))
                                                                         , Box::new(Expression::Number(3f64)))));

    assert_eq!(parse_exp(&mut make_lexer("1.1 ~= 7")), Ok(Expression::Binop(tokens::Keyword::NEQ
                                                                            , Box::new(Expression::Number(1.1f64))
                                                                            , Box::new(Expression::Number(7f64)))));
}

#[test]
fn test_exp_unop() {
    assert_eq!(parse_exp(&mut make_lexer("-3")), Ok(Expression::Unop(tokens::Keyword::MINUS
                                                                     , Box::new(Expression::Number(3f64)))));

    assert_eq!(parse_exp(&mut make_lexer("#7")), Ok(Expression::Unop(tokens::Keyword::HASH
                                                                            , Box::new(Expression::Number(7f64)))));
}

#[test]
fn test_exp_prefix() {
    assert_eq!(parse_exp(&mut make_lexer("Hello.world")), Ok(Expression::Indexing {
        object: Box::new(Expression::Id(vec!["Hello".to_owned()])),
        index: Box::new(Expression::String("world".to_owned())),
    }))
}

#[test]
fn test_exp_functiondef() {
    assert_eq!(parse_exp(&mut make_lexer("function f () body end")), Ok(make_assignment(vec!["f"],
                                                                                          Expression::Function {
                                                                                              params: vec![],
                                                                                              body: vec![],
                                                                                          })))
}

#[test]
fn test_exp_table_constructor() {
    assert_eq!(parse_exp(&mut make_lexer("{ [true] = false }")), Ok(Expression::TableConstructor(vec![Box::new(
        Expression::Assignment(
             Box::new(Expression::Boolean(true))
                , Box::new(Expression::Boolean(false))
        ))])));
}



