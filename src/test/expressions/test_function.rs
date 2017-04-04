use ast::lexer::Lexer;

use ast::expressions::*;

#[test]
#[should_panic]
fn test_empty_function() {
    let mut lexer = Lexer::new("".to_owned());

    function::from_lexer(&mut lexer);
}


#[test]
fn test_sample_function() {
    let mut lexer = Lexer::new("function f () body end".to_owned());

    assert_eq!(Expression::from_lexer(&mut lexer),
               Some(assignment::new(vec!["f".to_owned()],
                                    Expression::Function {
                                        params: vec![],
                                        body: vec![],
                                    })))
}

#[test]
fn test_complex_function() {
    let mut lexer = Lexer::new("function t.a.b.c.f () body end".to_owned());

    assert_eq!(Expression::from_lexer(&mut lexer),
               Some(assignment::new(vec!["t".to_owned(),
                                         "a".to_owned(),
                                         "b".to_owned(),
                                         "c".to_owned(),
                                         "f".to_owned()],
                                    Expression::Function {
                                        params: vec![],
                                        body: vec![],
                                    })))
}

#[test]
fn test_param_function() {
    let mut lexer = Lexer::new("function f (t, a, b, c) body end".to_owned());

    assert_eq!(Expression::from_lexer(&mut lexer),
               Some(assignment::new(vec!["f".to_owned()],
                                    Expression::Function {
                                        params: vec!["t".to_owned(),
                                                     "a".to_owned(),
                                                     "b".to_owned(),
                                                     "c".to_owned()],
                                        body: vec![],
                                    })))
}

#[test]
fn test_method() {
    let mut lexer = Lexer::new("function t.a:f(b, c) body end".to_owned());

    assert_eq!(Expression::from_lexer(&mut lexer),
               Some(assignment::new(vec!["t".to_owned(), "a".to_owned(), "f".to_owned()],
                                    Expression::Function {
                                        params: vec!["self".to_owned(),
                                                     "b".to_owned(),
                                                     "c".to_owned()],
                                        body: vec![],
                                    })))
}
