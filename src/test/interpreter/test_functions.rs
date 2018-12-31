use crate::ast::rules;

use super::utils::{ interpret_rule };

#[test]
fn test_closure_eval() {
    let (val, mut _env) = interpret_rule("function () break; end", rules::functiondef);
    assert_eq!(val, "Function { id: 0, parameters: [], varargs: false, body: Block { statements: [Break, Terminal(SEMICOLONS)], retstat: None }, env: 0 }");

    let (val, mut _env) = interpret_rule("function (b, c, ...) break; end", rules::functiondef);
    assert_eq!(val, r#"Function { id: 0, parameters: ["b", "c"], varargs: true, body: Block { statements: [Break, Terminal(SEMICOLONS)], retstat: None }, env: 0 }"#);
}

#[test]
fn test_function_eval() {
    let (_val, env) = interpret_rule("function t (...) break end", rules::stat);
    assert_eq!(env, r#"{"t": RefCell { value: Function { id: 0, parameters: [], varargs: true, body: Block { statements: [Break], retstat: None }, env: 0 } }}"#);

    let (_val, env) = interpret_rule("t = {}; function t:f(b, c, ...) break end", rules::block);
    assert_eq!(env, r#"{"t": RefCell { value: Table { id: 0, map: {String("f"): RefCell { value: Function { id: 0, parameters: ["self", "b", "c"], varargs: true, body: Block { statements: [Break], retstat: None }, env: 0 } }}, metatable: {}, border: 0 } }}"#);
}