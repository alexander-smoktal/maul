use ast::expressions::*;

use super::utils::*;

#[test]
fn test_var_id() {
    assert_eq!(variables::parse_var(&mut make_lexer("Hello")),
               Ok(Expression::Id(vec!["Hello".to_owned()])))
}

#[test]
fn test_var_name_resolution() {
    assert_eq!(variables::parse_var(&mut make_lexer("Hello.world")),
               Ok(Expression::Indexing {
                   object: Box::new(Expression::Id(vec!["Hello".to_owned()])),
                   index: Box::new(Expression::String("world".to_owned())),
               })
    )
}

#[test]
fn test_var_indexing() {
    assert_eq!(variables::parse_var(&mut make_lexer("(7)[...]")),
               Ok(Expression::Indexing {
                   object: Box::new(Expression::Number(7f64)),
                   index: Box::new(Expression::St(statements::Statement::Ellipsis)),
               })
    )
}

#[test]
fn test_assignment() {
    assert_eq!(variables::parse_assignment(&mut make_lexer("one, two, three = 1, true, false")),
               Ok(Expression::Expressions(vec![
                   Box::new(Expression::Assignment(Box::new(Expression::Id(vec!["one".to_string()])), Box::new(Expression::Number(1f64)))),
                   Box::new(Expression::Assignment(Box::new(Expression::Id(vec!["two".to_string()])), Box::new(Expression::Boolean(true)))),
                   Box::new(Expression::Assignment(Box::new(Expression::Id(vec!["three".to_string()])), Box::new(Expression::Boolean(false))))
               ]))
    )
}
