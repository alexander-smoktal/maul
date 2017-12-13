use ast::expressions::*;

use super::utils::*;

#[test]
fn test_var_id() {
    assert_eq!(variables::parse_var(&mut make_lexer("Hello")),
               Ok(Box::new(variables::Id(vec!["Hello".to_owned()])) as Box<expression::Expression>))
}

#[test]
fn test_var_name_resolution() {
    assert_eq!(variables::parse_var(&mut make_lexer("Hello.world")),
               Ok(Box::new(tables::Indexing {
                   object: Box::new(variables::Id(vec!["Hello".to_owned()])),
                   index: Box::new(primitives::String("world".to_owned())),
               }) as Box<expression::Expression>)
    )
}

#[test]
fn test_var_indexing() {
    assert_eq!(variables::parse_var(&mut make_lexer("(7)[...]")),
               Ok(Box::new(tables::Indexing {
                   object: Box::new(primitives::Number(7f64)),
                   index: Box::new(statements::Statement::Ellipsis),
               }) as Box<expression::Expression>)
    )
}

#[test]
fn test_assignment() {
    assert_eq!(variables::parse_assignment(&mut make_lexer("one, two, three = 1, true, false")),
               Ok(Box::new(util::Expressions(vec![
                   Box::new(variables::Assignment(Box::new(variables::Id(vec!["one".to_string()])), Box::new(primitives::Number(1f64)))),
                   Box::new(variables::Assignment(Box::new(variables::Id(vec!["two".to_string()])), Box::new(primitives::Boolean(true)))),
                   Box::new(variables::Assignment(Box::new(variables::Id(vec!["three".to_string()])), Box::new(primitives::Boolean(false))))
               ])) as Box<expression::Expression>)
    )
}
