use super::utils::parse_string;
use ast::rules;
/*#[test]
fn test_simple_statement() {
    assert_eq!(
        parse_statement(&mut make_lexer("break;")),
        Ok(exp!(Statement::Break))
    )
}*/

#[test]
fn test_return_statement() {
    assert_eq!(parse_string("return nil, false, 42;", rules::retstat),
        "[Single(Return(Some(Expressions([Nil, Boolean(false), Number(42.0)]))))]");

    assert_eq!(parse_string("return;", rules::retstat),
        "[Single(Return(None))]");
}
