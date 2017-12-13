use ast::expressions::*;

use super::utils::*;


#[test]
#[should_panic]
fn test_table_invalid() {
    assert_eq!(tables::parse_table_constructor(&mut make_lexer("1 + 3")), Ok(Box::new(util::Noop) as Box<expression::Expression>))
}

#[test]
fn test_table_constructor_simple() {
    assert_eq!(tables::parse_table_constructor(&mut make_lexer("{7}")), Ok(Box::new(tables::TableConstructor(vec![Box::new(primitives::Number(7f64))])) as Box<expression::Expression>));
}

#[test]
fn test_table_constructor_assignment() {
    assert_eq!(tables::parse_table_constructor(&mut make_lexer("{ Sashok = false }")), Ok(Box::new(tables::TableConstructor(vec![Box::new(
        variables::Assignment(
            Box::new(variables::Id(vec!["Sashok".to_string()]))
                , Box::new(primitives::Boolean(false))
        ))])) as Box<expression::Expression>));
}

#[test]
fn test_table_constructor_index_assignment() {
    assert_eq!(tables::parse_table_constructor(&mut make_lexer("{ [true] = false }")), Ok(Box::new(tables::TableConstructor(vec![Box::new(
        variables::Assignment(
             Box::new(primitives::Boolean(true))
                , Box::new(primitives::Boolean(false))
        ))])) as Box<expression::Expression>));
}

