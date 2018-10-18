use ast::rules;

use super::utils::interpret_rule;

#[test]
fn test_variable_simple() {
    let (_val, env) = interpret_rule("x = 3", rules::stat);
    assert_eq!(env, r#"{"x": RefCell { value: Number(3.0) }}"#);

    let (_val, env) = interpret_rule("x = 3, 2", rules::stat);
    assert_eq!(env, r#"{"x": RefCell { value: Number(3.0) }}"#);

    let (_val, env) = interpret_rule("x, y = 3, false", rules::stat);
    assert_eq!(env, r#"{"y": RefCell { value: Boolean(false) }, "x": RefCell { value: Number(3.0) }}"#);

    let (_val, env) = interpret_rule("x, y = 3", rules::stat);
    assert_eq!(env, r#"{"x": RefCell { value: Number(3.0) }, "y": RefCell { value: Nil }}"#);
}

/*#[test]
#[should_panic(expected = "Runtime error: Unknown variable 'hello'")]
fn test_variable_non_id() {
    let (_val, _env) = interpret_rule("7 = 3", rules::stat);
}*/