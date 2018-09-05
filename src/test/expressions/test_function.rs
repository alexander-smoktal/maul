use super::utils::parse_string;
use ast::rules;

#[test]
fn test_funcname() {
    assert_eq!(parse_string("a", rules::funcname), r#"[Single(Funcname { object: [Id("a")], method: None })]"#);
    assert_eq!(parse_string("a.b", rules::funcname), r#"[Single(Funcname { object: [Id("a"), Id("b")], method: None })]"#);
    assert_eq!(parse_string("a.b:c", rules::funcname), r#"[Single(Funcname { object: [Id("a"), Id("b")], method: Some(Id("c")) })]"#);
}

#[test]
fn test_func_args() {
    assert_eq!(parse_string("one", rules::parlist), r#"[Single(FunctionParameters { namelist: Some([Id("one")]), varargs: false })]"#);
    assert_eq!(parse_string("one, two", rules::parlist), r#"[Single(FunctionParameters { namelist: Some([Id("one"), Id("two")]), varargs: false })]"#);
    assert_eq!(parse_string("one, two, ...", rules::parlist), r#"[Single(FunctionParameters { namelist: Some([Id("one"), Id("two")]), varargs: true })]"#);
    assert_eq!(parse_string("...", rules::parlist), r#"[Single(FunctionParameters { namelist: None, varargs: true })]"#);
}

#[test]
#[should_panic]
fn test_invalid_args() {
    assert_eq!(parse_string("one, two,", rules::parlist), "[Single(Nil)]");
}

#[test]
#[should_panic]
fn test_multiple_varargs() {
    assert_eq!(parse_string("one, two, ..., ...", rules::parlist), "[Single(Nil)]");
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
        r#"[Single(Funcall { object: Id("func"), args: [Number(1.0), Number(5.0)], method: None })]"#);

    assert_eq!(parse_string("func()", rules::functioncall),
        r#"[Single(Funcall { object: Id("func"), args: [], method: None })]"#);

    assert_eq!(parse_string("obj.func()()", rules::functioncall),
        r#"[Single(Funcall { object: Funcall { object: Indexing { object: Id("obj"), index: Id("func") }, args: [], method: None }, args: [], method: None })]"#);

    assert_eq!(parse_string("obj:method(1, 5)", rules::functioncall),
        r#"[Single(Funcall { object: Id("obj"), args: [Number(1.0), Number(5.0)], method: Some(Id("method")) })]"#);

    assert_eq!(parse_string("obj:method()", rules::functioncall),
        r#"[Single(Funcall { object: Id("obj"), args: [], method: Some(Id("method")) })]"#);

    assert_eq!(parse_string("obj.func(1, 5)", rules::functioncall),
        r#"[Single(Funcall { object: Indexing { object: Id("obj"), index: Id("func") }, args: [Number(1.0), Number(5.0)], method: None })]"#);

    assert_eq!(parse_string("obj.func()", rules::functioncall),
        r#"[Single(Funcall { object: Indexing { object: Id("obj"), index: Id("func") }, args: [], method: None })]"#);

    assert_eq!(parse_string(r#"obj["func"](1, 5)"#, rules::functioncall),
        r#"[Single(Funcall { object: Indexing { object: Id("obj"), index: String("func") }, args: [Number(1.0), Number(5.0)], method: None })]"#);

    assert_eq!(parse_string(r#"obj["func"]()"#, rules::functioncall),
        r#"[Single(Funcall { object: Indexing { object: Id("obj"), index: String("func") }, args: [], method: None })]"#);
}

#[test]
#[should_panic]
fn test_invalid_funccall0() {

    assert_eq!(parse_string("a.b", rules::functioncall), "");
}
#[test]
#[should_panic]
fn test_invalid_funccall1() {
    assert_eq!(parse_string(r#"a["b"]"#, rules::functioncall), "");
}

#[test]
#[should_panic]
fn test_invalid_funccall2() {
    assert_eq!(parse_string("a(1).b", rules::functioncall), "");
}

#[test]
fn test_functioncall_rec_prefixexp() {
    assert_eq!(parse_string("(true)(1, 5)", rules::functioncall),
        r#"[Single(Funcall { object: Boolean(true), args: [Number(1.0), Number(5.0)], method: None })]"#);

    assert_eq!(parse_string("(true)()", rules::functioncall),
        r#"[Single(Funcall { object: Boolean(true), args: [], method: None })]"#);

    assert_eq!(parse_string("(true).func(1, 5)", rules::functioncall),
        r#"[Single(Funcall { object: Indexing { object: Boolean(true), index: Id("func") }, args: [Number(1.0), Number(5.0)], method: None })]"#);

    assert_eq!(parse_string(r#"(true)["func"]()"#, rules::functioncall),
        r#"[Single(Funcall { object: Indexing { object: Boolean(true), index: String("func") }, args: [], method: None })]"#);

    assert_eq!(parse_string("(true):method(1, 5)", rules::functioncall),
        r#"[Single(Funcall { object: Boolean(true), args: [Number(1.0), Number(5.0)], method: Some(Id("method")) })]"#);
}

#[test]
fn test_functioncall_rec_args() {
    assert_eq!(parse_string("func(1, 5)(3)", rules::functioncall),
        r#"[Single(Funcall { object: Funcall { object: Id("func"), args: [Number(1.0), Number(5.0)], method: None }, args: [Number(3.0)], method: None })]"#);

    assert_eq!(parse_string("func()(3)", rules::functioncall),
        r#"[Single(Funcall { object: Funcall { object: Id("func"), args: [], method: None }, args: [Number(3.0)], method: None })]"#);

    assert_eq!(parse_string("obj:method(1, 5)(3)", rules::functioncall),
        r#"[Single(Funcall { object: Funcall { object: Id("obj"), args: [Number(1.0), Number(5.0)], method: Some(Id("method")) }, args: [Number(3.0)], method: None })]"#);

    assert_eq!(parse_string("obj:method()(3)", rules::functioncall),
        r#"[Single(Funcall { object: Funcall { object: Id("obj"), args: [], method: Some(Id("method")) }, args: [Number(3.0)], method: None })]"#);

    assert_eq!(parse_string("obj.func1(1, 5).func2(3)", rules::functioncall),
        r#"[Single(Funcall { object: Indexing { object: Funcall { object: Indexing { object: Id("obj"), index: Id("func1") }, args: [Number(1.0), Number(5.0)], method: None }, index: Id("func2") }, args: [Number(3.0)], method: None })]"#);

    assert_eq!(parse_string("obj:method1():method2(3)", rules::functioncall),
        r#"[Single(Funcall { object: Funcall { object: Id("obj"), args: [], method: Some(Id("method1")) }, args: [Number(3.0)], method: Some(Id("method2")) })]"#);
}


#[test]
fn test_closure() {
    assert_eq!(parse_string("function () break; end", rules::functiondef), 
        "[Single(Closure { params: None, body: Block { statements: [Break, Terminal(SEMICOLONS)], retstat: None } })]");
    assert_eq!(parse_string("function (...) break; end", rules::functiondef), 
        "[Single(Closure { params: Some(FunctionParameters { namelist: None, varargs: true }), body: Block { statements: [Break, Terminal(SEMICOLONS)], retstat: None } })]");
    assert_eq!(parse_string("function (t, a, b, c) end", rules::functiondef), 
        r#"[Single(Closure { params: Some(FunctionParameters { namelist: Some([Id("t"), Id("a"), Id("b"), Id("c")]), varargs: false }), body: Block { statements: [], retstat: None } })]"#);
    assert_eq!(parse_string("function (b, c, ...) break; end", rules::functiondef), 
        r#"[Single(Closure { params: Some(FunctionParameters { namelist: Some([Id("b"), Id("c")]), varargs: true }), body: Block { statements: [Break, Terminal(SEMICOLONS)], retstat: None } })]"#);
    assert_eq!(parse_string("function (t, a, b, c) return 7; end", rules::functiondef), 
        r#"[Single(Closure { params: Some(FunctionParameters { namelist: Some([Id("t"), Id("a"), Id("b"), Id("c")]), varargs: false }), body: Block { statements: [], retstat: Some(Return(Some(Expressions([Number(7.0)])))) } })]"#);
}

/*#[test]
fn test_functiondef() {
    assert_eq!(parse_string("function f () break; end", rules::functiondef), 
        "");
    assert_eq!(parse_string("function t.a.b.c.f (...) break; end", rules::functiondef), 
        "");
    assert_eq!(parse_string("function f (t, a, b, c) break; end", rules::functiondef), 
        "");
    assert_eq!(parse_string("function t.a:f(b, c, ...) break; end", rules::functiondef), 
        "");
    assert_eq!(parse_string("function f (t, a, b, c) break; end", rules::functiondef), 
        "");
}
*/

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
