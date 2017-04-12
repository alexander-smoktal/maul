use ast::lexer::Lexer;

use ast::expressions::*;

#[test]
fn test_var_id() {
    let mut lexer = Lexer::new("Hello".to_owned());

    assert_eq!(assignment::parse_var(&mut lexer),
               Ok(Expression::Id("Hello".to_owned())))
}

#[test]
fn test_simple_indexing() {
    let mut lexer = Lexer::new("Hello.world".to_owned());

    assert_eq!(assignment::parse_var(&mut lexer),
               Ok(Expression::Indexing {
                   object: Box::new(Expression::Id("Hello".to_owned())),
                   index: Box::new(Expression::StringConstant("world".to_owned())),
               })
    )
}
