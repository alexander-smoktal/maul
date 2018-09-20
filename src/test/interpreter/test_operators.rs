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

// TODO: Test metatables operators
// Keyword::PLUS | Keyword::MINUS | Keyword::MUL | Keyword::DIV | Keyword::FLOORDIV | Keyword::MOD | Keyword::POW
#[test]
fn test_binop_arithmetic() {
    let (val, mut _env) = interpret_rule("1 + 3", rules::exp); assert_eq!(val, "Number(4.0)");
    let (val, mut _env) = interpret_rule("1 - 3", rules::exp); assert_eq!(val, "Number(-2.0)");
    let (val, mut _env) = interpret_rule("0 * 3", rules::exp); assert_eq!(val, "Number(0.0)");
    let (val, mut _env) = interpret_rule("1 / 4", rules::exp); assert_eq!(val, "Number(0.25)");
    let (val, mut _env) = interpret_rule("11 // 3", rules::exp); assert_eq!(val, "Number(3.0)");
    let (val, mut _env) = interpret_rule("8 % 3", rules::exp); assert_eq!(val, "Number(2.0)");
    let (val, mut _env) = interpret_rule("2 ^ 3", rules::exp); assert_eq!(val, "Number(8.0)");
}

#[test]
fn test_binop_arithmetic_conversion() {
    let (val, mut _env) = interpret_rule(r#"1 + "3""#, rules::exp); assert_eq!(val, "Number(4.0)");
    let (val, mut _env) = interpret_rule(r#""1" - 3"#, rules::exp); assert_eq!(val, "Number(-2.0)");
    let (val, mut _env) = interpret_rule(r#""0.8" / "4""#, rules::exp); assert_eq!(val, "Number(0.2)");
}

#[test]
#[should_panic(expected = r#"Runtime error: Can't convert string "Hello" to apply + operator"#)]
fn test_binop_arithmetic_invalid_conversion() {
    let (val, mut _env) = interpret_rule(r#"1 + "Hello""#, rules::exp); assert_eq!(val, "Number(4.0)");
}

// Keyword::LESS | Keyword::LEQ | Keyword::GREATER | Keyword::GEQ | Keyword::EQ | Keyword::NEQ
#[test]
fn test_binop_comparison_numbers() {
    let (val, mut _env) = interpret_rule("1 < 3", rules::exp); assert_eq!(val, "Boolean(true)");
    let (val, mut _env) = interpret_rule("3 < 3", rules::exp); assert_eq!(val, "Boolean(false)");

    let (val, mut _env) = interpret_rule("-1 <= 3", rules::exp); assert_eq!(val, "Boolean(true)");
    let (val, mut _env) = interpret_rule("3 <= 3", rules::exp); assert_eq!(val, "Boolean(true)");
    let (val, mut _env) = interpret_rule("5 <= 4", rules::exp); assert_eq!(val, "Boolean(false)");

    let (val, mut _env) = interpret_rule("1 > 3", rules::exp); assert_eq!(val, "Boolean(false)");
    let (val, mut _env) = interpret_rule("3 > 3", rules::exp); assert_eq!(val, "Boolean(false)");
    let (val, mut _env) = interpret_rule("4 > 3", rules::exp); assert_eq!(val, "Boolean(true)");

    let (val, mut _env) = interpret_rule("-4 >= -3", rules::exp); assert_eq!(val, "Boolean(false)");
    let (val, mut _env) = interpret_rule("-3 >= -3", rules::exp); assert_eq!(val, "Boolean(true)");
    let (val, mut _env) = interpret_rule("-1 >= -3", rules::exp); assert_eq!(val, "Boolean(true)");

    let (val, mut _env) = interpret_rule("3 == 3", rules::exp); assert_eq!(val, "Boolean(true)");
    let (val, mut _env) = interpret_rule("1 == 3", rules::exp); assert_eq!(val, "Boolean(false)");

    let (val, mut _env) = interpret_rule("3 ~= 3", rules::exp); assert_eq!(val, "Boolean(false)");
    let (val, mut _env) = interpret_rule("1 ~= 3", rules::exp); assert_eq!(val, "Boolean(true)");
}

#[test]
fn test_binop_comparison_strings() {
    let (val, mut _env) = interpret_rule(r#""a" < "b""#, rules::exp); assert_eq!(val, "Boolean(true)");
    let (val, mut _env) = interpret_rule(r#""a" < "ab""#, rules::exp); assert_eq!(val, "Boolean(true)");
    let (val, mut _env) = interpret_rule(r#""b" < "a""#, rules::exp); assert_eq!(val, "Boolean(false)");
    let (val, mut _env) = interpret_rule(r#""ab" < "a""#, rules::exp); assert_eq!(val, "Boolean(false)");
    let (val, mut _env) = interpret_rule(r#""a" < "a""#, rules::exp); assert_eq!(val, "Boolean(false)");

    let (val, mut _env) = interpret_rule(r#""a" <= "b""#, rules::exp); assert_eq!(val, "Boolean(true)");
    let (val, mut _env) = interpret_rule(r#""a" <= "ab""#, rules::exp); assert_eq!(val, "Boolean(true)");
    let (val, mut _env) = interpret_rule(r#""b" <= "a""#, rules::exp); assert_eq!(val, "Boolean(false)");
    let (val, mut _env) = interpret_rule(r#""ab" <= "a""#, rules::exp); assert_eq!(val, "Boolean(false)");
    let (val, mut _env) = interpret_rule(r#""a" <= "a""#, rules::exp); assert_eq!(val, "Boolean(true)");

    let (val, mut _env) = interpret_rule(r#""a" == "b""#, rules::exp); assert_eq!(val, "Boolean(false)");
    let (val, mut _env) = interpret_rule(r#""a" == "ab""#, rules::exp); assert_eq!(val, "Boolean(false)");
    let (val, mut _env) = interpret_rule(r#""b" == "a""#, rules::exp); assert_eq!(val, "Boolean(false)");
    let (val, mut _env) = interpret_rule(r#""ab" == "a""#, rules::exp); assert_eq!(val, "Boolean(false)");
    let (val, mut _env) = interpret_rule(r#""a" == "a""#, rules::exp); assert_eq!(val, "Boolean(true)");

    let (val, mut _env) = interpret_rule(r#""a" ~= "b""#, rules::exp); assert_eq!(val, "Boolean(true)");
    let (val, mut _env) = interpret_rule(r#""a" ~= "ab""#, rules::exp); assert_eq!(val, "Boolean(true)");
    let (val, mut _env) = interpret_rule(r#""b" ~= "a""#, rules::exp); assert_eq!(val, "Boolean(true)");
    let (val, mut _env) = interpret_rule(r#""ab" ~= "a""#, rules::exp); assert_eq!(val, "Boolean(true)");
    let (val, mut _env) = interpret_rule(r#""a" ~= "a""#, rules::exp); assert_eq!(val, "Boolean(false)");
}

#[test]
fn test_binop_comparison_diff_types() {
    let (val, mut _env) = interpret_rule(r#""1" ~= 1"#, rules::exp); assert_eq!(val, "Boolean(false)");
}

// Keyword::SOR | Keyword::TILDA | Keyword::SAND | Keyword::SHRIGHT | Keyword::SHLEFT
#[test]
fn test_binop_bitwise() {
    let (val, mut _env) = interpret_rule("1 | 3", rules::exp); assert_eq!(val, "Number(3.0)");
    let (val, mut _env) = interpret_rule("1 | 2", rules::exp); assert_eq!(val, "Number(3.0)");

    let (val, mut _env) = interpret_rule("1 & 3", rules::exp); assert_eq!(val, "Number(1.0)");
    let (val, mut _env) = interpret_rule("1 & 2", rules::exp); assert_eq!(val, "Number(0.0)");

    let (val, mut _env) = interpret_rule("1 ~ 3", rules::exp); assert_eq!(val, "Number(2.0)");
    let (val, mut _env) = interpret_rule("1 ~ 2", rules::exp); assert_eq!(val, "Number(3.0)");

    let (val, mut _env) = interpret_rule("1 << 3", rules::exp); assert_eq!(val, "Number(8.0)");
    let (val, mut _env) = interpret_rule("1 << 2", rules::exp); assert_eq!(val, "Number(4.0)");

    let (val, mut _env) = interpret_rule("8 >> 3", rules::exp); assert_eq!(val, "Number(1.0)");
    let (val, mut _env) = interpret_rule("8 >> 2", rules::exp); assert_eq!(val, "Number(2.0)");
}

#[test]
#[should_panic(expected = r#"Runtime error: Bitwise operator can be applied only to numbers. Got String("1") and Number(3.0)"#)]
fn test_binop_bitwise_invalid() {
    interpret_rule(r#""1" | 3"#, rules::exp);
}

// Keyword::OR | Keyword::AND
#[test]
fn test_binop_boolean() {
    let (val, mut _env) = interpret_rule("true or false", rules::exp); assert_eq!(val, "Boolean(true)");
    let (val, mut _env) = interpret_rule("false or true", rules::exp); assert_eq!(val, "Boolean(true)");
    let (val, mut _env) = interpret_rule("true or true", rules::exp); assert_eq!(val, "Boolean(true)");
    let (val, mut _env) = interpret_rule("false or false", rules::exp); assert_eq!(val, "Boolean(false)");

    let (val, mut _env) = interpret_rule("nil or false", rules::exp); assert_eq!(val, "Boolean(false)");
    let (val, mut _env) = interpret_rule("false or nil", rules::exp); assert_eq!(val, "Boolean(false)");

    let (val, mut _env) = interpret_rule("1 or false", rules::exp); assert_eq!(val, "Boolean(true)");
    let (val, mut _env) = interpret_rule(r#"false or "nil""#, rules::exp); assert_eq!(val, "Boolean(true)");

    let (val, mut _env) = interpret_rule("true and false", rules::exp); assert_eq!(val, "Boolean(false)");
    let (val, mut _env) = interpret_rule("false and true", rules::exp); assert_eq!(val, "Boolean(false)");
    let (val, mut _env) = interpret_rule("true and true", rules::exp); assert_eq!(val, "Boolean(true)");
    let (val, mut _env) = interpret_rule("false and false", rules::exp); assert_eq!(val, "Boolean(false)");

    let (val, mut _env) = interpret_rule("nil and true", rules::exp); assert_eq!(val, "Boolean(false)");
    let (val, mut _env) = interpret_rule("true and nil", rules::exp); assert_eq!(val, "Boolean(false)");

    let (val, mut _env) = interpret_rule("1 and true", rules::exp); assert_eq!(val, "Boolean(true)");
    let (val, mut _env) = interpret_rule(r#"true and "nil""#, rules::exp); assert_eq!(val, "Boolean(true)");
}

// Keyword::DOT2
#[test]
fn test_binop_concat() {
    let (val, mut _env) = interpret_rule(r#""Hello ".."world""#, rules::exp); assert_eq!(val, r#"String("Hello world")"#);
    let (val, mut _env) = interpret_rule(r#""Hello "..1"#, rules::exp); assert_eq!(val, r#"String("Hello 1")"#);
    let (val, mut _env) = interpret_rule(r#"1 .." world""#, rules::exp); assert_eq!(val, r#"String("1 world")"#);
    let (val, mut _env) = interpret_rule("1 .. 2", rules::exp); assert_eq!(val, r#"String("12")"#);
}