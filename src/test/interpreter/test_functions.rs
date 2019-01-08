use crate::ast::rules;

use super::utils::{interpret_rule, interpret_rule_env};

#[test]
fn test_closure_eval() {
    let (val, mut _env) = interpret_rule("function () break; end", rules::functiondef);
    assert_eq!(val, "Function { id: 1, parameters: [], varargs: false, body: Block { statements: [Break, Terminal(SEMICOLONS)], retstat: None }, env: 0 }");

    let (val, mut _env) = interpret_rule("function (b, c, ...) break; end", rules::functiondef);
    assert_eq!(val, r#"Function { id: 1, parameters: ["b", "c"], varargs: true, body: Block { statements: [Break, Terminal(SEMICOLONS)], retstat: None }, env: 0 }"#);
}

#[test]
fn test_function_eval() {
    let (_val, env) = interpret_rule("function t (...) break end", rules::stat);
    assert_eq!(env, r#"{"t": RefCell { value: Function { id: 1, parameters: [], varargs: true, body: Block { statements: [Break], retstat: None }, env: 0 } }}"#);

    let (_val, env) = interpret_rule("t = {}; function t:f(b, c, ...) break end", rules::block);
    assert_eq!(env, r#"{"t": RefCell { value: Table { id: 1, map: {String("f"): RefCell { value: Function { id: 2, parameters: ["self", "b", "c"], varargs: true, body: Block { statements: [Break], retstat: None }, env: 0 } }}, metatable: {}, border: 0 } }}"#);
}

#[test]
fn test_functioncall_eval() {
    let (_, mut env) = interpret_rule("function sum1(x) return x + 1; end", rules::stat);

    let (val, _) = interpret_rule_env("sum1(5)", rules::functioncall, &mut env);
    assert_eq!(val, "Number(6.0)");
}

#[test]
fn test_method_eval() {
    let (_, mut env) = interpret_rule("tab = {}; tab.x = 5;", rules::block);
    let (_, mut env) = interpret_rule_env("function tab:sum(x) return x + self.x; end", rules::stat, &mut env);

    let (val, _) = interpret_rule_env("tab:sum(5)", rules::functioncall, &mut env);
    assert_eq!(val, "Number(10.0)");
}

#[test]
fn test_varags_eval() {
    let (_, mut env) = interpret_rule("function args(...) return arg; end", rules::stat);

    let (val, _) = interpret_rule_env("args(5)", rules::functioncall, &mut env);
    assert_eq!(val, "Reference(RefCell { value: Vector([Number(5.0)]) })");

    let (_, mut env) = interpret_rule("function args(a, b, ...) return arg; end", rules::stat);

    let (val, _) = interpret_rule_env("args(1, 2, 3, 4)", rules::functioncall, &mut env);
    assert_eq!(val, "Reference(RefCell { value: Vector([Number(3.0), Number(4.0)]) })");
}
