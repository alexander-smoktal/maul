use ast::expressions::*;

use super::utils::*;

#[test]
fn test_var_id() {
    assert_eq!(
        variables::parse_var(&mut make_lexer("Hello")),
        Ok(exp!(variables::Id(vec!["Hello".to_owned()])))
    )
}

#[test]
fn test_var_name_resolution() {
    assert_eq!(
        variables::parse_var(&mut make_lexer("Hello.world")),
        Ok(exp!(tables::Indexing {
            object: exp!(variables::Id(vec!["Hello".to_owned()])),
            index: exp!(primitives::String("world".to_owned())),
        }))
    )
}

#[test]
fn test_var_indexing() {
    assert_eq!(
        variables::parse_var(&mut make_lexer("(7)[...]")),
        Ok(exp!(tables::Indexing {
            object: exp!(primitives::Number(7f64)),
            index: exp!(statements::Statement::Ellipsis),
        }))
    )
}

#[test]
fn test_assignment() {
    assert_eq!(
        variables::parse_assignment(&mut make_lexer("one, two, three = 1, true, false")),
        Ok(exp!(util::Expressions(vec![
            exp!(variables::Assignment(
                exp!(variables::Id(vec!["one".to_string()])),
                exp!(primitives::Number(1f64)),
            )),
            exp!(variables::Assignment(
                exp!(variables::Id(vec!["two".to_string()])),
                exp!(primitives::Boolean(true)),
            )),
            exp!(variables::Assignment(
                exp!(variables::Id(vec!["three".to_string()])),
                exp!(primitives::Boolean(false)),
            )),
        ])))
    )
}
