use ast::rules;

use super::utils::interpret_rule;

#[test]
fn test_primitives() {
    let (val, mut _env) = interpret_rule("nil", rules::exp);
    assert_eq!(val, "Nil");

    let (val, _env) = interpret_rule("true", rules::exp);
    assert_eq!(val, "Boolean(true)");

    let (val, _env) = interpret_rule("false", rules::exp);
    assert_eq!(val, "Boolean(false)");

    let (val, _env) = interpret_rule("42.4", rules::exp);
    assert_eq!(val, "Number(42.4)");

    let (val, _env) = interpret_rule(r#""Hello world""#, rules::exp);
    assert_eq!(val, r#"String("Hello world")"#);
}