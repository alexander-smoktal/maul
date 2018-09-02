use ast::parser::rules;
use super::utils::parse_string;

#[test]
fn test_exp_terminals() {
    assert_eq!(parse_string("nil", rules::exp), "[Single(Nil)]");
    assert_eq!(parse_string("false", rules::exp), "[Single(Boolean(false))]");
    assert_eq!(parse_string("true", rules::exp), "[Single(Boolean(true))]");
    assert_eq!(parse_string("...", rules::exp), "[Single(Ellipsis)]");
    assert_eq!(parse_string("42.42", rules::exp), "[Single(Number(42.42))]");
    assert_eq!(parse_string(r#""Hello""#, rules::exp), r#"[Single(String("Hello"))]"#);
}
    
#[test]
fn test_explist() {
    assert_eq!(parse_string("nil, false", rules::explist), "[Single(Expressions([Nil, Boolean(false)]))]");
    assert_eq!(parse_string("nil, false, 42", rules::explist), "[Single(Expressions([Nil, Boolean(false), Number(42.0)]))]");
}

#[test]
fn test_exp_unop() {
    assert_eq!(parse_string("-3", rules::exp), "[Single(Unop(MINUS, Number(3.0)))]");
    assert_eq!(parse_string("#7", rules::exp), "[Single(Unop(HASH, Number(7.0)))]");
    assert_eq!(parse_string("~false", rules::exp), "[Single(Unop(TILDA, Boolean(false)))]");
}


#[test]
fn test_exp_binop() {
    assert_eq!(parse_string("1 - 3", rules::exp), "[Single(Binop(MINUS, Number(1.0), Number(3.0)))]");
    assert_eq!(parse_string("1 - 3 + 4", rules::exp), "[Single(Binop(MINUS, Number(1.0), Binop(PLUS, Number(3.0), Number(4.0))))]");
    assert_eq!(parse_string("-1 - -3", rules::exp), "[Single(Binop(MINUS, Unop(MINUS, Number(1.0)), Unop(MINUS, Number(3.0))))]");
}

/*
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
