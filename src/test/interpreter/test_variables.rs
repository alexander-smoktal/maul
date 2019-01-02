use crate::ast::rules;

use super::utils::{interpret_rule, interpret_rule_env};

#[test]
fn test_variable_simple() {
    let (_val, env) = interpret_rule("x = 3", rules::stat);
    assert_eq!(env, r#"{"x": RefCell { value: Number(3.0) }}"#);

    let (_val, env) = interpret_rule("x = 3, 2", rules::stat);
    assert_eq!(env, r#"{"x": RefCell { value: Number(3.0) }}"#);

    let (_val, env) = interpret_rule("x, y = 3, false", rules::stat);
    assert_eq!(
        env,
        r#"{"x": RefCell { value: Number(3.0) }, "y": RefCell { value: Boolean(false) }}"#
    );

    let (_val, env) = interpret_rule("x, y = 3", rules::stat);
    assert_eq!(
        env,
        r#"{"x": RefCell { value: Number(3.0) }, "y": RefCell { value: Nil }}"#
    );
}

#[test]
fn test_variable_table() {
    let (_val, env) = interpret_rule("x = {}", rules::stat);
    assert_eq!(
        env,
        r#"{"x": RefCell { value: Table { id: 1, map: {}, metatable: {}, border: 0 } }}"#
    );

    let (_val, mut env) = interpret_rule("x = {y = 5, [5] = false}", rules::stat);

    let (val, mut env) = interpret_rule_env("x.y", rules::var, &mut env);
    assert_eq!(val, "Reference(RefCell { value: Number(5.0) })");

    let (val, _env) = interpret_rule_env("x[5]", rules::var, &mut env);
    assert_eq!(val, "Reference(RefCell { value: Boolean(false) })");
}

#[test]
fn test_variable_table_change() {
    let (_val, mut env) = interpret_rule("x = {y = 5}", rules::stat);

    let (val, mut env) = interpret_rule_env("x.y", rules::var, &mut env);
    assert_eq!(val, "Reference(RefCell { value: Number(5.0) })");

    let (_val, mut env) = interpret_rule_env("x.y = 7", rules::stat, &mut env);
    let (val, _env) = interpret_rule_env("x.y", rules::var, &mut env);
    assert_eq!(val, "Reference(RefCell { value: Number(7.0) })");
}

#[test]
#[should_panic(expected = "Runtime error: Attempt to index `Nil` value")]
fn test_variable_non_id() {
    let (_val, _env) = interpret_rule("x.y", rules::var);
}
