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


#[test]
fn test_funcall_sample() {
    assert_eq!(function::parse_funcall(&mut make_lexer("Account.withdraw(100.00)")),
               Ok(Expression::Funcall {
                   function: Box::new(Expression::Indexing {
                       object: Box::new(Expression::Id(vec!["Account".to_string()])),
                       index: Box::new(Expression::String("withdraw".to_string())),
                   }),
                   args: Box::new(Expression::Expressions(vec![Box::new(Expression::Number(100f64))]))
               })
    )
}

#[test]
fn test_funcall_complex() {
    assert_eq!(function::parse_funcall(&mut make_lexer("Account:withdraw(100.00)")),
               Ok(Expression::Funcall {
                   function: Box::new(Expression::Indexing {
                       object: Box::new(Expression::Id(vec!["Account".to_string()])),
                       index: Box::new(Expression::String("withdraw".to_string())),
                   }),
                   args: Box::new(Expression::Expressions(vec![
                       Box::new(Expression::Id(vec!["Account".to_string()])),
                       Box::new(Expression::Number(100f64))]))
               })
    )
}


/*#[test]
fn test_fib() {
    match function::parse_funcdef(&mut make_lexer("
      function a.b:fib(n)
        N=N+1
        if n<2 then
          return n
        else
          return a.b.fib(n-1) + a.b.fib(n-2)
        end
      end"
    )) {
        Ok(yoba) => assert_eq!(yoba, Expression::Noop),
        Err(err) => err.complain()
    }
}*/
