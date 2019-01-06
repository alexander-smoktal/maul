use crate::ast::rules;

use super::utils::interpret_rule;

#[test]
fn test_break_for() {
    let (_val, env) = interpret_rule("y = 3 for i = 0, 10 do y = y + i; if y >= 5 then break end end", rules::block);
    assert_eq!(env, r#"{"y": RefCell { value: Number(6.0) }}"#);
}

#[test]
fn test_break_while() {
    let (_val, env) = interpret_rule("y = 3 while y < 10 do y = y + 1; if y >= 5 then break end end", rules::block);
    assert_eq!(env, r#"{"y": RefCell { value: Number(5.0) }}"#);
}

#[test]
fn test_break_repeat() {
    let (_val, env) = interpret_rule("y = 3 repeat y = y + 1; if y > 5 then break end until y > 10", rules::block);
    assert_eq!(env, r#"{"y": RefCell { value: Number(6.0) }}"#);
}
