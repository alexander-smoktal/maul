use super::utils::parse_string;
use ast::rules;

#[test]
fn test_funcname() {
    assert_eq!(parse_string("a", rules::funcname), r#"[Single(Funcname { names: [Id("a")], this: false })]"#);
    assert_eq!(parse_string("a.b", rules::funcname), r#"[Single(Funcname { names: [Id("a"), Id("b")], this: false })]"#);
    assert_eq!(parse_string("a.b:c", rules::funcname), r#"[Single(Funcname { names: [Id("a"), Id("b"), Id("c")], this: true })]"#);
}

#[test]
#[should_panic]
fn test_invalid_functions0() {
    
    assert_eq!(parse_string("a.", rules::funcname), "");
}
#[test]
#[should_panic]
fn test_invalid_functions1() {
    assert_eq!(parse_string("a.:c", rules::funcname), "");
}

#[test]
#[should_panic]
fn test_invalid_functions2() {
    assert_eq!(parse_string("a:", rules::funcname), "");
}

#[test]
fn test_functioncall() {
    assert_eq!(parse_string("func(1, 5)", rules::functioncall), 
        r#"[Single(Funcall { function: Id("func"), args: [Number(1.0), Number(5.0)], method: None })]"#);

    assert_eq!(parse_string("func()", rules::functioncall), 
        r#"[Single(Funcall { function: Id("func"), args: [], method: None })]"#);

    assert_eq!(parse_string("obj:method(1, 5)", rules::functioncall), 
        r#"[Single(Funcall { function: Id("obj"), args: [Number(1.0), Number(5.0)], method: Some(Id("method")) })]"#);

    assert_eq!(parse_string("obj:method()", rules::functioncall), 
        r#"[Single(Funcall { function: Id("obj"), args: [], method: Some(Id("method")) })]"#);

    assert_eq!(parse_string("obj.func(1, 5)", rules::functioncall), 
        r#"[Single(Funcall { function: Indexing { object: Id("obj"), index: Id("func") }, args: [Number(1.0), Number(5.0)], method: None })]"#);

    assert_eq!(parse_string("obj.func()", rules::functioncall), 
        r#"[Single(Funcall { function: Indexing { object: Id("obj"), index: Id("func") }, args: [], method: None })]"#);

    assert_eq!(parse_string(r#"obj["func"](1, 5)"#, rules::functioncall), 
        r#"[Single(Funcall { function: Indexing { object: Id("obj"), index: String("func") }, args: [Number(1.0), Number(5.0)], method: None })]"#);

    assert_eq!(parse_string(r#"obj["func"]()"#, rules::functioncall), 
        r#"[Single(Funcall { function: Indexing { object: Id("obj"), index: String("func") }, args: [], method: None })]"#);
}

#[test]
fn test_functioncall_rec_prefixexp() {
    assert_eq!(parse_string("(true)(1, 5)", rules::functioncall), 
        r#"[Single(Funcall { function: Boolean(true), args: [Number(1.0), Number(5.0)], method: None })]"#);

    assert_eq!(parse_string("(true)()", rules::functioncall), 
        r#"[Single(Funcall { function: Boolean(true), args: [], method: None })]"#);

    assert_eq!(parse_string("(true).func(1, 5)", rules::functioncall), 
        r#"[Single(Funcall { function: Indexing { object: Boolean(true), index: Id("func") }, args: [Number(1.0), Number(5.0)], method: None })]"#);

    assert_eq!(parse_string(r#"(true)["func"]()"#, rules::functioncall), 
        r#"[Single(Funcall { function: Indexing { object: Boolean(true), index: String("func") }, args: [], method: None })]"#);

    assert_eq!(parse_string("(true):method(1, 5)", rules::functioncall), 
        r#"[Single(Funcall { function: Boolean(true), args: [Number(1.0), Number(5.0)], method: Some(Id("method")) })]"#);
}

#[test]
fn test_functioncall_rec_args() {
    assert_eq!(parse_string("func(1, 5)(3)", rules::functioncall), 
        r#"[Single(Funcall { function: Funcall { function: Id("func"), args: [Number(1.0), Number(5.0)], method: None }, args: [Number(3.0)], method: None })]"#);

    assert_eq!(parse_string("func()(3)", rules::functioncall), 
        r#"[Single(Funcall { function: Funcall { function: Id("func"), args: [], method: None }, args: [Number(3.0)], method: None })]"#);

    assert_eq!(parse_string("obj:method(1, 5)(3)", rules::functioncall), 
        r#"[Single(Funcall { function: Funcall { function: Id("obj"), args: [Number(1.0), Number(5.0)], method: Some(Id("method")) }, args: [Number(3.0)], method: None })]"#);

    assert_eq!(parse_string("obj:method()(3)", rules::functioncall), 
        r#"[Single(Funcall { function: Funcall { function: Id("obj"), args: [], method: Some(Id("method")) }, args: [Number(3.0)], method: None })]"#);

    assert_eq!(parse_string("obj.func1(1, 5).func2(3)", rules::functioncall), 
        r#"[Single(Funcall { function: Indexing { object: Funcall { function: Indexing { object: Id("obj"), index: Id("func1") }, args: [Number(1.0), Number(5.0)], method: None }, index: Id("func2") }, args: [Number(3.0)], method: None })]"#);

    assert_eq!(parse_string("obj:method1():method2(3)", rules::functioncall), 
        r#"[Single(Funcall { function: Funcall { function: Id("obj"), args: [], method: Some(Id("method1")) }, args: [Number(3.0)], method: Some(Id("method2")) })]"#);
}

/*
#[test]
fn test_empty_function() {
    assert_eq!(
        function::parse_funcdef(&mut make_lexer("")),
        Err(error::Error::new(
            tokens::Token::eof(),
            "Expected 'function' keyword at the function start. Got: Token \
            { token: None, row: 0, column: 0 }",
        ))
    )
}


#[test]
fn test_sample_function() {
    assert_eq!(
        function::parse_funcdef(&mut make_lexer("function f () break; end")),
        Ok(make_assignment(
            vec!["f"],
            exp!(function::Function {
                params: vec![],
                body: exp!(common::Expressions(
                    vec![exp!(Statement::Break), exp!(common::Noop)],
                )),
            }),
        ))
    )
}

#[test]
fn test_complex_function() {
    assert_eq!(
        function::parse_funcdef(&mut make_lexer("function t.a.b.c.f () break; end")),
        Ok(make_assignment(
            vec!["t", "a", "b", "c", "f"],
            exp!(function::Function {
                params: vec![],
                body: exp!(common::Expressions(
                    vec![exp!(Statement::Break), exp!(common::Noop)],
                )),
            }),
        ))
    )
}

#[test]
fn test_param_function() {
    assert_eq!(
        function::parse_funcdef(&mut make_lexer("function f (t, a, b, c) break; end")),
        Ok(make_assignment(
            vec!["f"],
            exp!(function::Function {
                params: vec![
                    "t".to_owned(),
                    "a".to_owned(),
                    "b".to_owned(),
                    "c".to_owned(),
                ],
                body: exp!(common::Expressions(
                    vec![exp!(Statement::Break), exp!(common::Noop)],
                )),
            }),
        ))
    )
}

#[test]
fn test_method() {
    assert_eq!(
        function::parse_funcdef(&mut make_lexer("function t.a:f(b, c) break; end")),
        Ok(make_assignment(
            vec!["t", "a", "f"],
            exp!(function::Function {
                params: vec!["self".to_owned(), "b".to_owned(), "c".to_owned()],
                body: exp!(common::Expressions(
                    vec![exp!(Statement::Break), exp!(common::Noop)],
                )),
            }),
        ))
    )
}


#[test]
fn test_funcall_sample() {
    assert_eq!(
        function::parse_funcall(&mut make_lexer("Account.withdraw(100.00)")),
        Ok(exp!(function::Funcall {
            function: exp!(tables::Indexing {
                object: exp!(variables::Id(vec!["Account".to_string()])),
                index: exp!(primitives::String("withdraw".to_string())),
            }),
            args: exp!(common::Expressions(vec![exp!(primitives::Number(100f64))])),
        }))
    )
}

#[test]
fn test_funcall_nested() {
    assert_eq!(
        function::parse_funcall(&mut make_lexer("Customer.account.withdraw(100.00)")),
        Ok(exp!(function::Funcall {
            function: exp!(tables::Indexing {
                object: exp!(tables::Indexing {
                    object: exp!(variables::Id(vec!["Customer".to_string()])),
                    index: exp!(primitives::String("account".to_string())),
                }),
                index: exp!(primitives::String("withdraw".to_string())),
            }),
            args: exp!(common::Expressions(vec![exp!(primitives::Number(100f64))])),
        }))
    )
}

#[test]
fn test_funcall_complex() {
    assert_eq!(
        function::parse_funcall(&mut make_lexer("Account:withdraw(100.00)")),
        Ok(exp!(function::Funcall {
            function: exp!(tables::Indexing {
                object: exp!(variables::Id(vec!["Account".to_string()])),
                index: exp!(primitives::String("withdraw".to_string())),
            }),
            args: exp!(common::Expressions(vec![
                exp!(variables::Id(vec!["Account".to_string()])),
                exp!(primitives::Number(100f64)),
            ])),
        }))
    )
}*/


//#[test]
//fn test_fib() {
//    assert_eq!(
//        function::parse_funcdef(&mut make_lexer(
//            "
//      function a.b:fib(n)
//        N=N+1
//        if n<2 then
//          return n
//        else
//          return a.b.fib(n-1) + a.b.fib(n-2)
//        end
//      end",
//        )),
//        Ok(make_assignment(
//            vec!["t", "a", "f"],
//            exp!(function::Function {
//                params: vec!["self".to_owned(), "b".to_owned(), "c".to_owned()],
//                body: exp!(common::Expressions(
//                    vec![exp!(Statement::Break), exp!(common::Noop)],
//                )),
//            }),
//        ))
//    )
//}
