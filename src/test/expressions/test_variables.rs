use ast::lexer::Lexer;

use ast::expressions::*;

#[test]
fn test_var_id() {
    let mut lexer = Lexer::new("Hello".to_owned());

    assert_eq!(variables::parse_var(&mut lexer),
               Ok(Expression::Id(vec!["Hello".to_owned()])))
}

#[test]
fn test_simple_indexing() {
    let mut lexer = Lexer::new("Hello.world".to_owned());

    assert_eq!(variables::parse_var(&mut lexer),
               Ok(Expression::Indexing {
                   object: Box::new(Expression::Id(vec!["Hello".to_owned()])),
                   index: Box::new(Expression::String("world".to_owned())),
               })
    )
}
