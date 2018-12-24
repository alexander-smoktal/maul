use super::utils::parse_string;
use crate::ast::rules;

#[test]
fn test_funcname() {
    assert_eq!(
        parse_string("a", rules::funcname),
        r#"[Single(Funcname { object: [Id("a")], method: None })]"#
    );
    assert_eq!(
        parse_string("a.b", rules::funcname),
        r#"[Single(Funcname { object: [Id("a"), String("b")], method: None })]"#
    );
    assert_eq!(
        parse_string("a.b:c", rules::funcname),
        r#"[Single(Funcname { object: [Id("a"), String("b")], method: Some(String("c")) })]"#
    );
}

#[test]
fn test_func_args() {
    assert_eq!(
        parse_string("one", rules::parlist),
        r#"[Single(FunctionParameters { namelist: [String("one")], varargs: false })]"#
    );
    assert_eq!(
        parse_string("one, two", rules::parlist),
        r#"[Single(FunctionParameters { namelist: [String("one"), String("two")], varargs: false })]"#
    );
    assert_eq!(
        parse_string("one, two, ...", rules::parlist),
        r#"[Single(FunctionParameters { namelist: [String("one"), String("two")], varargs: true })]"#
    );
    assert_eq!(
        parse_string("...", rules::parlist),
        r#"[Single(FunctionParameters { namelist: [], varargs: true })]"#
    );
}

#[test]
#[should_panic]
fn test_invalid_args() {
    parse_string("one, two,", rules::parlist);
}

#[test]
#[should_panic]
fn test_multiple_varargs() {
    parse_string("one, two, ..., ...", rules::parlist);
}

#[test]
#[should_panic]
fn test_invalid_functions0() {
    parse_string("a.", rules::funcname);
}
#[test]
#[should_panic]
fn test_invalid_functions1() {
    parse_string("a.:c", rules::funcname);
}

#[test]
#[should_panic]
fn test_invalid_functions2() {
    parse_string("a:", rules::funcname);
}

#[test]
fn test_functioncall() {
    assert_eq!(parse_string("func(1, 5)", rules::functioncall),
        r#"[Single(Funcall { object: Id("func"), args: [Number(1.0), Number(5.0)], method: None })]"#);

    assert_eq!(
        parse_string("func()", rules::functioncall),
        r#"[Single(Funcall { object: Id("func"), args: [], method: None })]"#
    );

    assert_eq!(parse_string("obj.func()()", rules::functioncall),
        r#"[Single(Funcall { object: Funcall { object: Indexing { object: Id("obj"), index: String("func") }, args: [], method: None }, args: [], method: None })]"#);

    assert_eq!(parse_string("obj:method(1, 5)", rules::functioncall),
        r#"[Single(Funcall { object: Id("obj"), args: [Number(1.0), Number(5.0)], method: Some(String("method")) })]"#);

    assert_eq!(
        parse_string("obj:method()", rules::functioncall),
        r#"[Single(Funcall { object: Id("obj"), args: [], method: Some(String("method")) })]"#
    );

    assert_eq!(parse_string("obj.func(1, 5)", rules::functioncall),
        r#"[Single(Funcall { object: Indexing { object: Id("obj"), index: String("func") }, args: [Number(1.0), Number(5.0)], method: None })]"#);

    assert_eq!(parse_string("obj.func()", rules::functioncall),
        r#"[Single(Funcall { object: Indexing { object: Id("obj"), index: String("func") }, args: [], method: None })]"#);

    assert_eq!(parse_string(r#"obj["func"](1, 5)"#, rules::functioncall),
        r#"[Single(Funcall { object: Indexing { object: Id("obj"), index: String("func") }, args: [Number(1.0), Number(5.0)], method: None })]"#);

    assert_eq!(parse_string(r#"obj["func"]()"#, rules::functioncall),
        r#"[Single(Funcall { object: Indexing { object: Id("obj"), index: String("func") }, args: [], method: None })]"#);
}

#[test]
#[should_panic]
fn test_invalid_funccall0() {
    parse_string("a.b", rules::functioncall);
}
#[test]
#[should_panic]
fn test_invalid_funccall1() {
    parse_string(r#"a["b"]"#, rules::functioncall);
}

#[test]
#[should_panic]
fn test_invalid_funccall2() {
    parse_string("a(1).b", rules::functioncall);
}

#[test]
fn test_functioncall_rec_prefixexp() {
    assert_eq!(parse_string("(true)(1, 5)", rules::functioncall),
        r#"[Single(Funcall { object: Boolean(true), args: [Number(1.0), Number(5.0)], method: None })]"#);

    assert_eq!(
        parse_string("(true)()", rules::functioncall),
        r#"[Single(Funcall { object: Boolean(true), args: [], method: None })]"#
    );

    assert_eq!(parse_string("(true).func(1, 5)", rules::functioncall),
        r#"[Single(Funcall { object: Indexing { object: Boolean(true), index: String("func") }, args: [Number(1.0), Number(5.0)], method: None })]"#);

    assert_eq!(parse_string(r#"(true)["func"]()"#, rules::functioncall),
        r#"[Single(Funcall { object: Indexing { object: Boolean(true), index: String("func") }, args: [], method: None })]"#);

    assert_eq!(parse_string("(true):method(1, 5)", rules::functioncall),
        r#"[Single(Funcall { object: Boolean(true), args: [Number(1.0), Number(5.0)], method: Some(String("method")) })]"#);
}

#[test]
fn test_functioncall_rec_args() {
    assert_eq!(parse_string("func(1, 5)(3)", rules::functioncall),
        r#"[Single(Funcall { object: Funcall { object: Id("func"), args: [Number(1.0), Number(5.0)], method: None }, args: [Number(3.0)], method: None })]"#);

    assert_eq!(parse_string("func()(3)", rules::functioncall),
        r#"[Single(Funcall { object: Funcall { object: Id("func"), args: [], method: None }, args: [Number(3.0)], method: None })]"#);

    assert_eq!(parse_string("obj:method(1, 5)(3)", rules::functioncall),
        r#"[Single(Funcall { object: Funcall { object: Id("obj"), args: [Number(1.0), Number(5.0)], method: Some(String("method")) }, args: [Number(3.0)], method: None })]"#);

    assert_eq!(parse_string("obj:method()(3)", rules::functioncall),
        r#"[Single(Funcall { object: Funcall { object: Id("obj"), args: [], method: Some(String("method")) }, args: [Number(3.0)], method: None })]"#);

    assert_eq!(parse_string("obj.func1(1, 5).func2(3)", rules::functioncall),
        r#"[Single(Funcall { object: Indexing { object: Funcall { object: Indexing { object: Id("obj"), index: String("func1") }, args: [Number(1.0), Number(5.0)], method: None }, index: String("func2") }, args: [Number(3.0)], method: None })]"#);

    assert_eq!(parse_string("obj:method1():method2(3)", rules::functioncall),
        r#"[Single(Funcall { object: Funcall { object: Id("obj"), args: [], method: Some(String("method1")) }, args: [Number(3.0)], method: Some(String("method2")) })]"#);
}

#[test]
fn test_closure() {
    assert_eq!(parse_string("function () break; end", rules::functiondef),
        "[Single(Closure { params: None, body: Block { statements: [Break, Terminal(SEMICOLONS)], retstat: None } })]");
    assert_eq!(parse_string("function (...) break; end", rules::functiondef),
        "[Single(Closure { params: Some(FunctionParameters { namelist: [], varargs: true }), body: Block { statements: [Break, Terminal(SEMICOLONS)], retstat: None } })]");
    assert_eq!(parse_string("function (t, a, b, c) end", rules::functiondef),
        r#"[Single(Closure { params: Some(FunctionParameters { namelist: [String("t"), String("a"), String("b"), String("c")], varargs: false }), body: Block { statements: [], retstat: None } })]"#);
    assert_eq!(parse_string("function (b, c, ...) break; end", rules::functiondef),
        r#"[Single(Closure { params: Some(FunctionParameters { namelist: [String("b"), String("c")], varargs: true }), body: Block { statements: [Break, Terminal(SEMICOLONS)], retstat: None } })]"#);
    assert_eq!(parse_string("function (t, a, b, c) return 7; end", rules::functiondef),
        r#"[Single(Closure { params: Some(FunctionParameters { namelist: [String("t"), String("a"), String("b"), String("c")], varargs: false }), body: Block { statements: [], retstat: Some(Return(Some(Expressions([Number(7.0)])))) } })]"#);
}

#[test]
fn test_functiondef() {
    assert_eq!(parse_string("function f () break; end", rules::stat),
        r#"[Single(Function { name: Funcname { object: [Id("f")], method: None }, body: Closure { params: None, body: Block { statements: [Break, Terminal(SEMICOLONS)], retstat: None } } })]"#);
    assert_eq!(parse_string("function t.a.b.c.f (...) break end", rules::stat),
        r#"[Single(Function { name: Funcname { object: [Id("t"), String("a"), String("b"), String("c"), String("f")], method: None }, body: Closure { params: Some(FunctionParameters { namelist: [], varargs: true }), body: Block { statements: [Break], retstat: None } } })]"#);
    assert_eq!(parse_string("function f (t, a, b, c) break end", rules::stat),
        r#"[Single(Function { name: Funcname { object: [Id("f")], method: None }, body: Closure { params: Some(FunctionParameters { namelist: [String("t"), String("a"), String("b"), String("c")], varargs: false }), body: Block { statements: [Break], retstat: None } } })]"#);
    assert_eq!(parse_string("function t.a:f(b, c, ...) break end", rules::stat),
        r#"[Single(Function { name: Funcname { object: [Id("t"), String("a")], method: Some(String("f")) }, body: Closure { params: Some(FunctionParameters { namelist: [String("b"), String("c")], varargs: true }), body: Block { statements: [Break], retstat: None } } })]"#);
}

#[test]
fn test_local_function() {
    assert_eq!(parse_string("local function f () break end", rules::stat),
        r#"[Single(Local(Function { name: Id("f"), body: Closure { params: None, body: Block { statements: [Break], retstat: None } } }))]"#);
}

#[test]
#[should_panic]
fn test_local_function_invalid() {
    parse_string("local function t.a.b.c.f () break end", rules::stat);
}

#[test]
fn test_fib() {
    assert_eq!(
       parse_string(
    "\
     function a.b:fib(n) \
       N=N+1 \
       if n<2 then \
         return n \
       else \
         return a.b.fib(n-1) + a.b.fib(n-2) \
       end \
     end", rules::chunk),
       r#"[Single(Block { statements: [Function { name: Funcname { object: [Id("a"), String("b")], method: Some(String("fib")) }, body: Closure { params: Some(FunctionParameters { namelist: [String("n")], varargs: false }), body: Block { statements: [Assignment { varlist: [Id("N")], explist: [Binop(PLUS, Id("N"), Number(1.0))] }, IfBlock { conditions: [IfCondition { condition: Binop(LESS, Id("n"), Number(2.0)), block: Block { statements: [], retstat: Some(Return(Some(Expressions([Id("n")])))) } }], else_block: Some(Block { statements: [], retstat: Some(Return(Some(Expressions([Binop(PLUS, Funcall { object: Indexing { object: Indexing { object: Id("a"), index: String("b") }, index: String("fib") }, args: [Binop(MINUS, Id("n"), Number(1.0))], method: None }, Funcall { object: Indexing { object: Indexing { object: Id("a"), index: String("b") }, index: String("fib") }, args: [Binop(MINUS, Id("n"), Number(2.0))], method: None })])))) }) }], retstat: None } } }], retstat: None })]"#);
}
