use ast::lexer::*;
use ast::expressions::*;
use ast::expressions::statements::*;
use error;

use super::utils::*;

#[test]
fn test_empty_function() {
    assert_eq!(function::parse_funcdef(&mut make_lexer("")),
               Err(error::Error::new(tokens::Token::eof(),
                                     "Expected 'function' keyword at the function start. Got: Token { token: None, row: 0, column: 0 }")))
}


#[test]
fn test_sample_function() {
    assert_eq!(function::parse_funcdef(&mut make_lexer("function f () break; end")),
               Ok(make_assignment(vec!["f"],
                                  Expression::Function {
                                      params: vec![],
                                      body: Box::new(Expression::Expressions(vec![Box::new(Expression::St(Statement::Break))]))
                                  })))
}

#[test]
fn test_complex_function() {
    assert_eq!(function::parse_funcdef(&mut make_lexer("function t.a.b.c.f () break; end")),
               Ok(make_assignment(vec!["t", "a", "b", "c", "f"],
                                  Expression::Function {
                                      params: vec![],
                                      body: Box::new(Expression::Expressions(vec![Box::new(Expression::St(Statement::Break))]))
                                  })))
}

#[test]
fn test_param_function() {
    assert_eq!(function::parse_funcdef(&mut make_lexer("function f (t, a, b, c) break; end")),
               Ok(make_assignment(vec!["f"],
                                  Expression::Function {
                                      params: vec!["t".to_owned(),
                                                   "a".to_owned(),
                                                   "b".to_owned(),
                                                   "c".to_owned()],
                                      body: Box::new(Expression::Expressions(vec![Box::new(Expression::St(Statement::Break))]))
                                  })))
}

#[test]
fn test_method() {
    assert_eq!(function::parse_funcdef(&mut make_lexer("function t.a:f(b, c) break; end")),
               Ok(make_assignment(vec!["t", "a", "f"],
                                  Expression::Function {
                                      params: vec!["self".to_owned(),
                                                   "b".to_owned(),
                                                   "c".to_owned()],
                                      body: Box::new(Expression::Expressions(vec![Box::new(Expression::St(Statement::Break))]))
                                  })))
}
