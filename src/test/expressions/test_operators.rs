use super::utils::parse_string;
use ast::rules;

// or
// and
// <     >     <=    >=    ~=    ==
// |
// ~
// &
// <<    >>
// ..
// +     -
// *     /     //    %
// ^

#[test]
fn test_operator_simple() {
    assert_eq!(parse_string("1 ^ 5", rules::exp), "[Single(Binop(POW, Number(1.0), Number(5.0)))]");
    assert_eq!(parse_string("1 * 5", rules::exp), "[Single(Binop(MUL, Number(1.0), Number(5.0)))]");
    assert_eq!(parse_string("true or false", rules::exp), "[Single(Binop(OR, Boolean(true), Boolean(false)))]");
}

#[test]
fn test_operator_rep() {
    assert_eq!(parse_string("1 ^ 5 ^ 3", rules::exp), "[Single(Binop(POW, Binop(POW, Number(1.0), Number(5.0)), Number(3.0)))]");
    assert_eq!(parse_string("1 * 5 / 2", rules::exp), "[Single(Binop(DIV, Binop(MUL, Number(1.0), Number(5.0)), Number(2.0)))]");
    assert_eq!(parse_string("true or false or true", rules::exp), "[Single(Binop(OR, Binop(OR, Boolean(true), Boolean(false)), Boolean(true)))]");
}

#[test]
fn test_operator_precedence() {
    assert_eq!(parse_string("1 ^ 5 * 3", rules::exp), "[Single(Binop(MUL, Binop(POW, Number(1.0), Number(5.0)), Number(3.0)))]");
    assert_eq!(parse_string("1 * 5 ^ 3", rules::exp), "[Single(Binop(MUL, Number(1.0), Binop(POW, Number(5.0), Number(3.0))))]");
    assert_eq!(parse_string("1 * 5 + 3 * 9", rules::exp), "[Single(Binop(PLUS, Binop(MUL, Number(1.0), Number(5.0)), Binop(MUL, Number(3.0), Number(9.0))))]");
    assert_eq!(parse_string("1 - 5 * 3 - 9", rules::exp), "[Single(Binop(MINUS, Binop(MINUS, Number(1.0), Binop(MUL, Number(5.0), Number(3.0))), Number(9.0)))]");
    assert_eq!(parse_string("1 + 5 * 3 * 9", rules::exp), "[Single(Binop(PLUS, Number(1.0), Binop(MUL, Binop(MUL, Number(5.0), Number(3.0)), Number(9.0))))]");
    assert_eq!(parse_string("1 * 5 * 3 - 9", rules::exp), "[Single(Binop(MINUS, Binop(MUL, Binop(MUL, Number(1.0), Number(5.0)), Number(3.0)), Number(9.0)))]");
    assert_eq!(parse_string("1 - 5 ^ 3 * 9", rules::exp), "[Single(Binop(MINUS, Number(1.0), Binop(MUL, Binop(POW, Number(5.0), Number(3.0)), Number(9.0))))]");
}

#[test]
#[should_panic]
fn test_operator_invalid() {
    parse_string("1 ^ 5 *", rules::exp);
}