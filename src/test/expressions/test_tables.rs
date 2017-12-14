use ast::expressions::*;

use super::utils::*;


#[test]
#[should_panic]
fn test_table_invalid() {
    assert_eq!(
        tables::parse_table_constructor(&mut make_lexer("1 + 3")),
        Ok(exp!(common::Noop))
    )
}

#[test]
fn test_table_constructor_simple() {
    assert_eq!(
        tables::parse_table_constructor(&mut make_lexer("{7}")),
        Ok(exp!(tables::TableConstructor(
            vec![exp!(primitives::Number(7f64))],
        )))
    );
}

#[test]
fn test_table_constructor_assignment() {
    assert_eq!(
        tables::parse_table_constructor(&mut make_lexer("{ Sashok = false }")),
        Ok(exp!(tables::TableConstructor(vec![
            exp!(variables::Assignment(
                exp!(variables::Id(vec!["Sashok".to_string()])),
                exp!(primitives::Boolean(false)),
            )),
        ])))
    );
}

#[test]
fn test_table_constructor_index_assignment() {
    assert_eq!(
        tables::parse_table_constructor(&mut make_lexer("{ [true] = false }")),
        Ok(exp!(tables::TableConstructor(vec![
            exp!(variables::Assignment(
                exp!(primitives::Boolean(true)),
                exp!(primitives::Boolean(false)),
            )),
        ])))
    );
}
