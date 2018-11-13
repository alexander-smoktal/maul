use ast::rules;

use super::utils::{ interpret_rule };

#[test]
fn test_do_block() {
    let (_val, env) = interpret_rule("y = 3; do y = 5 end", rules::block);
    assert_eq!(env, r#"{"y": RefCell { value: Number(5.0) }}"#);
}