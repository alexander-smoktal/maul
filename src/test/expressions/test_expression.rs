//use ast::lexer::tokens;
use ast::expressions::*;
//use ast::expressions::statements::*;

use super::utils::*;

#[test]
fn test_exp_terminals() {
    assert_eq!(parse_string("nil"), sexp!(primitives::Nil));
    assert_eq!(parse_string("false"), sexp!(primitives::Boolean(false)));
    assert_eq!(parse_string("true"), sexp!(primitives::Boolean(true)));
    //assert_eq!(parse_string("..."), sexp!(Statement::Ellipsis));
    assert_eq!(parse_string("42.42"),sexp!(primitives::Number(42.42f64)));
    assert_eq!(parse_string(r#""Hello""#), sexp!(primitives::String("Hello".to_string())));
}


#[test]
fn test_multiple_expressions() {
    assert_eq!(parse_string("nil, false"), sexp!(expression::Expressions(vec![
        exp!(primitives::Nil),
        exp!(primitives::Boolean(false))
    ])));

    assert_eq!(parse_string("nil, false, 42"), sexp!(expression::Expressions(vec![
        exp!(primitives::Nil),
        exp!(expression::Expressions(vec![
            exp!(primitives::Boolean(false)),
            exp!(primitives::Number(42f64))]))
    ])));
}
/*
#[test]
fn test_exp_binop() {
    assert_eq!(
        parse_exp(&mut make_lexer("1 + 3")),
        Ok(exp!(operators::Binop(
            tokens::Keyword::PLUS,
            exp!(primitives::Number(1f64)),
            exp!(primitives::Number(3f64)),
        )))
    );

    assert_eq!(
        parse_exp(&mut make_lexer("1.1 ~= 7")),
        Ok(exp!(operators::Binop(
            tokens::Keyword::NEQ,
            exp!(primitives::Number(1.1f64)),
            exp!(primitives::Number(7f64)),
        )))
    );
}

#[test]
fn test_exp_unop() {
    assert_eq!(
        parse_exp(&mut make_lexer("-3")),
        Ok(exp!(operators::Unop(
            tokens::Keyword::MINUS,
            exp!(primitives::Number(3f64)),
        )))
    );

    assert_eq!(
        parse_exp(&mut make_lexer("#7")),
        Ok(exp!(operators::Unop(
            tokens::Keyword::HASH,
            exp!(primitives::Number(7f64)),
        )))
    );
}

#[test]
fn test_exp_prefix() {
    assert_eq!(
        parse_exp(&mut make_lexer("Hello.world")),
        Ok(exp!(tables::Indexing {
            object: exp!(variables::Id(vec!["Hello".to_owned()])),
            index: exp!(primitives::String("world".to_owned())),
        }))
    )
}

#[test]
fn test_exp_functiondef() {
    assert_eq!(
        parse_exp(&mut make_lexer("function f () break end")),
        Ok(make_assignment(
            vec!["f"],
            exp!(function::Function {
                params: vec![],
                body: exp!(common::Expressions(vec![exp!(Statement::Break)])),
            }),
        ))
    )
}

#[test]
fn test_exp_table_constructor() {
    assert_eq!(
        parse_exp(&mut make_lexer("{ [true] = false }")),
        Ok(exp!(tables::TableConstructor(vec![
            exp!(variables::Assignment(
                exp!(primitives::Boolean(true)),
                exp!(primitives::Boolean(false)),
            )),
        ])))
    );
}*/
