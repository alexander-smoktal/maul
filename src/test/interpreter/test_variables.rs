use ast::rules;

use super::utils::{ interpret_rule, interpret_rule_env };

#[test]
fn test_variable_simple() {
    let (_val, env) = interpret_rule("x = 3", rules::stat);
    assert_eq!(env, r#"{"x": RefCell { value: Number(3.0) }}"#);

    let (_val, env) = interpret_rule("x = 3, 2", rules::stat);
    assert_eq!(env, r#"{"x": RefCell { value: Number(3.0) }}"#);

    let (_val, env) = interpret_rule("x, y = 3, false", rules::stat);
    assert_eq!(env, r#"{"x": RefCell { value: Number(3.0) }, "y": RefCell { value: Boolean(false) }}"#);

    let (_val, env) = interpret_rule("x, y = 3", rules::stat);
    assert_eq!(env, r#"{"x": RefCell { value: Number(3.0) }, "y": RefCell { value: Nil }}"#);
}

#[test]
fn test_variable_table() {
    let (_val, env) = interpret_rule("x = {}", rules::stat);
    assert_eq!(env, r#"{"x": RefCell { value: Table { id: 0, map: {}, metatable: {}, border: 0 } }}"#);

    let (_val, env) = interpret_rule("x = {y = 5, [5] = false}", rules::stat);

    println!("YOBA: {}", env);

    let (val, env) = interpret_rule_env("x.y", rules::var, env);
    assert_eq!(val, "");

    let (val, _env) = interpret_rule_env("x[5]", rules::var, env);
    assert_eq!(val, "");
}

/*#[test]
#[should_panic(expected = "Runtime error: Unknown variable 'hello'")]
fn test_variable_non_id() {
    let (_val, _env) = interpret_rule("x = 3", rules::stat);
}*/