use ast::rules;

use super::utils::interpret_rule;

#[test]
fn test_unop_minus() {
    let (val, mut _env) = interpret_rule("-7", rules::exp);
    assert_eq!(val, "Number(-7.0)");
}

#[test]
#[should_panic(expected = "Runtime error: Can't negate Boolean(true) value")]
fn test_unop_minus_invalid() {
    interpret_rule("-true", rules::exp);
}

#[test]
fn test_unop_not() {
    let (val, mut _env) = interpret_rule("not true", rules::exp);
    assert_eq!(val, "Boolean(false)");

    let (val, mut _env) = interpret_rule("not false", rules::exp);
    assert_eq!(val, "Boolean(true)");

    let (val, mut _env) = interpret_rule("not nil", rules::exp);
    assert_eq!(val, "Boolean(true)");

    let (val, mut _env) = interpret_rule("not 7", rules::exp);
    assert_eq!(val, "Boolean(false)");
}

#[test]
fn test_unop_len() {
    let (val, mut _env) = interpret_rule(r#"#"Hello world""#, rules::exp);
    assert_eq!(val, "Number(11.0)");

    // TODO
    /*let (val, mut _env) = interpret_rule(r#"{1, 2, 3}"#, rules::exp);
    assert_eq!(val, "Number(11)");*/
}

#[test]
#[should_panic(expected = "Runtime error: Can't get length of Number(7.0) value")]
fn test_unop_len_invalid() {
    interpret_rule(r#"#7"#, rules::exp);
}

#[test]
fn test_unop_bitwise_not() {
    let (val, mut _env) = interpret_rule("~1100", rules::exp);
    assert_eq!(val, "Number(-1101.0)");
}

#[test]
#[should_panic(expected = "Runtime error: Can't apply bitwise not to Boolean(true) value")]
fn test_unop_bitwise_not_invalid() {
    interpret_rule("~true", rules::exp);
}