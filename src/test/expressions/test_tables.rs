use ast::expressions::*;

use super::utils::*;


#[test]
#[should_panic]
fn test_table_invalid() {
    assert_eq!(tables::parse_table_constructor(&mut make_lexer("1 + 3")), Ok(Expression::Stub))
}

#[test]
fn test_table_constructor_simple() {
    assert_eq!(tables::parse_table_constructor(&mut make_lexer("{7}")), Ok(Expression::TableConstructor(vec![Box::new(Expression::Number(7f64))])));
}

#[test]
fn test_table_constructor_assignment() {
    assert_eq!(tables::parse_table_constructor(&mut make_lexer("{ Sashok = false }")), Ok(Expression::TableConstructor(vec![Box::new(
        Expression::Assignment(
            Box::new(Expression::Id(vec!["Sashok".to_string()]))
                , Box::new(Expression::Boolean(false))
        ))])));
}

#[test]
fn test_table_constructor_index_assignment() {
    assert_eq!(tables::parse_table_constructor(&mut make_lexer("{ [true] = false }")), Ok(Expression::TableConstructor(vec![Box::new(
        Expression::Assignment(
             Box::new(Expression::Boolean(true))
                , Box::new(Expression::Boolean(false))
        ))])));
}

