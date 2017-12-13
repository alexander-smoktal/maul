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
                                  exp!(function::Function {
                                      params: vec![],
                                      body: exp!(util::Expressions(vec![exp!(Statement::Break), exp!(util::Noop)]))
                                  }))))
}

#[test]
fn test_complex_function() {
    assert_eq!(function::parse_funcdef(&mut make_lexer("function t.a.b.c.f () break; end")),
               Ok(make_assignment(vec!["t", "a", "b", "c", "f"],
                                  exp!(function::Function {
                                      params: vec![],
                                      body: exp!(util::Expressions(vec![exp!(Statement::Break), exp!(util::Noop)]))
                                  }))))
}

#[test]
fn test_param_function() {
    assert_eq!(function::parse_funcdef(&mut make_lexer("function f (t, a, b, c) break; end")),
               Ok(make_assignment(vec!["f"],
                                  exp!(function::Function {
                                      params: vec!["t".to_owned(),
                                                   "a".to_owned(),
                                                   "b".to_owned(),
                                                   "c".to_owned()],
                                      body: exp!(util::Expressions(vec![exp!(Statement::Break), exp!(util::Noop)]))
                                  }))))
}

#[test]
fn test_method() {
    assert_eq!(function::parse_funcdef(&mut make_lexer("function t.a:f(b, c) break; end")),
               Ok(make_assignment(vec!["t", "a", "f"],
                                  exp!(function::Function {
                                      params: vec!["self".to_owned(),
                                                   "b".to_owned(),
                                                   "c".to_owned()],
                                      body: exp!(util::Expressions(vec![exp!(Statement::Break), exp!(util::Noop)]))
                                  }))))
}


#[test]
fn test_funcall_sample() {
    assert_eq!(function::parse_funcall(&mut make_lexer("Account.withdraw(100.00)")),
               Ok(exp!(function::Funcall {
                   function: exp!(tables::Indexing {
                       object: exp!(variables::Id(vec!["Account".to_string()])),
                       index: exp!(primitives::String("withdraw".to_string())),
                   }),
                   args: exp!(util::Expressions(vec![exp!(primitives::Number(100f64))]))
               }))
    )
}

#[test]
fn test_funcall_complex() {
    assert_eq!(function::parse_funcall(&mut make_lexer("Account:withdraw(100.00)")),
               Ok(exp!(function::Funcall {
                   function: exp!(tables::Indexing {
                       object: exp!(variables::Id(vec!["Account".to_string()])),
                       index: exp!(primitives::String("withdraw".to_string())),
                   }),
                   args: exp!(util::Expressions(vec![
                       exp!(variables::Id(vec!["Account".to_string()])),
                       exp!(primitives::Number(100f64))]))
               }))
    )
}


#[test]
#[should_panic]
fn test_fib() {
    assert_eq!(function::parse_funcdef(&mut make_lexer("
      function a.b:fib(n)
        N=N+1
        if n<2 then
          return n
        else
          return a.b.fib(n-1) + a.b.fib(n-2)
        end
      end"
    )), Ok(exp!(util::Noop)))
}
