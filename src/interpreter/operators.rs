use std::clone::Clone;

use crate::ast::expressions::operators;
use crate::ast::lexer::tokens::Keyword;

use crate::interpreter::{self, environment, types};
use crate::utils;

impl interpreter::Eval for operators::Unop {
    // unop ::= ‘-’ | not | ‘#’ | ‘~’
    // Keyword::MINUS, Keyword::NOT, Keyword::HASH, Keyword::TILDA
    fn eval(&self, env: &mut utils::Shared<environment::Environment>) -> types::Type {
        let value = self.1.eval(env);

        // Keyword
        match self.0 {
            Keyword::MINUS => match_type!(&value,
                types::Type::Number(number) => types::Type::Number(-number),
                types::Type::Table { ref metatable, .. } => {
                    if let Some(metamethod) = metatable.get("__unm") {
                        metamethod.call(vec![&value])
                    } else {
                        self.runtime_error(format!("{:?} metatable doesn't contain `__unm` function", value))
                    }
                },
                _ => self.runtime_error(format!("Can't negate {:?} value", value))
            ),
            Keyword::NOT => types::Type::Boolean(!value.as_bool()),
            Keyword::HASH => match_type!(&value,
                types::Type::String(string) => types::Type::Number(string.as_bytes().len() as f64),
                types::Type::Table { border, ref metatable, .. } => {
                    if let Some(metamethod) = metatable.get("__len") {
                        metamethod.call(vec![&value])
                    } else {
                        types::Type::Number(*border as f64)
                    }
                },
                _ => {
                    self.runtime_error(format!("Can't get length of {:?} value", value));
                }
            ),
            Keyword::TILDA => match_type!(&value,
                types::Type::Number(number) => types::Type::Number(!(*number as i64) as f64),
                _ => self.runtime_error(format!("Can't apply bitwise not to {:?} value", value))
            ),
            _ => panic!("Should never happen"),
        }
    }
}

fn eval_ariphmetic(
    exp: &interpreter::Eval,
    op: &Keyword,
    left: types::Type,
    right: types::Type,
) -> types::Type {
    // Function to convert value for arithmetic operation
    let normalize = |value, op| -> f64 {
        match_type!(&value,
            types::Type::Number(number) => *number,
            types::Type::String(string) => {
                if let Ok(number) = string.parse::<f64>() {
                    number
                } else {
                    exp.runtime_error(format!("Can't convert string {:?} to apply {} operator", string, op))
                }
            },
            _ => exp.runtime_error(format!("Can't apply {} operator to {:?} value", op, value))
        )
    };

    macro_rules! metatable_binop {
        ($mt_key: tt, $op: tt, $function: expr) => {{
            // TODO: unref table
            if let types::Type::Table { ref metatable, .. } = left {
                if let Some(metamethod) = metatable.get($mt_key) {
                    return metamethod.call(vec![&left, &right]);
                }
            }

            return types::Type::Number($function(normalize(left, $op), normalize(right, $op)));
        }};
    }

    match op {
        Keyword::PLUS => metatable_binop!("__add", "+", |left, right| left + right),
        Keyword::MINUS => metatable_binop!("__sub", "-", |left, right| left - right),
        Keyword::MUL => metatable_binop!("__mul", "*", |left, right| left * right),
        Keyword::DIV => metatable_binop!("__div", "/", |left, right| left / right),
        Keyword::FLOORDIV => metatable_binop!("__idiv", "//", |left: f64, right: f64| (left
            / right)
            .floor()),
        Keyword::MOD => metatable_binop!("__mod", "%", |left: f64, right: f64| left % right),
        Keyword::POW => metatable_binop!("__pow", "^", |left: f64, right: f64| left.powf(right)),
        _ => panic!("Should never happen"),
    }
}

fn eval_equivalence(
    exp: &interpreter::Eval,
    op: &Keyword,
    left: types::Type,
    right: types::Type,
) -> types::Type {
    fn not(value: types::Type) -> types::Type {
        types::Type::Boolean(!value.as_bool())
    }

    match_type!((&left, &right),
        (types::Type::Number(leftnum), types::Type::Number(rightnum)) => {
            match op {
                Keyword::LESS => types::Type::Boolean(leftnum < rightnum),
                Keyword::LEQ => types::Type::Boolean(leftnum <= rightnum),
                Keyword::GREATER => types::Type::Boolean(leftnum > rightnum),
                Keyword::GEQ => types::Type::Boolean(leftnum >= rightnum),
                Keyword::EQ => types::Type::Boolean(leftnum == rightnum),
                Keyword::NEQ => types::Type::Boolean(leftnum != rightnum),
                _ => panic!("Should never happen")
            }
        },
        (types::Type::String(leftnum), types::Type::String(rightnum)) => {
            match op {
                Keyword::LESS => types::Type::Boolean(leftnum < rightnum),
                Keyword::LEQ => types::Type::Boolean(leftnum <= rightnum),
                Keyword::GREATER => types::Type::Boolean(leftnum > rightnum),
                Keyword::GEQ => types::Type::Boolean(leftnum >= rightnum),
                Keyword::EQ => types::Type::Boolean(leftnum == rightnum),
                Keyword::NEQ => types::Type::Boolean(leftnum != rightnum),
                _ => panic!("Should never happen")
            }
        },
        (types::Type::Table { ref metatable, .. }, _) => {
            macro_rules! metatable_binop {
                ($mt_key: tt, $op: tt) => {
                    if let Some(metamethod) = metatable.get($mt_key) {
                        metamethod.call(vec![&left, &right])
                    } else {
                        exp.runtime_error(format!("Can't compare values {:?} and {:?} with {} operator", &left, &right, $op))
                    }
                }
            }


            match op {
                Keyword::LESS => metatable_binop!("__lt", "<"),
                Keyword::LEQ => metatable_binop!("__le", "<="),
                Keyword::GREATER => not(metatable_binop!("__le", ">")),
                Keyword::GEQ => not(metatable_binop!("__lt", ">=")),
                Keyword::EQ => metatable_binop!("__eq", "=="),
                Keyword::NEQ => not(metatable_binop!("__eq", "~=")),
                _ => panic!("Should never happen")
            }
        },
        // TODO: Function comparison not implemented
        _ => types::Type::Boolean(false)
    )
}

fn eval_bitwise(
    exp: &interpreter::Eval,
    op: &Keyword,
    left: types::Type,
    right: types::Type,
) -> types::Type {
    macro_rules! metatable_binop {
        ($mt_key: tt, $op: tt, $function: expr) => ({
            match_type!(&left,
                types::Type::Table { ref metatable, .. } => {
                    if let Some(metamethod) = metatable.get($mt_key) {
                        return metamethod.call(vec![&left, &right])
                    }
                },
                _ => ()
            );

            match_type!((&left, &right),
                (types::Type::Number(leftnum), types::Type::Number(rightnum)) => {
                    return types::Type::Number($function(*leftnum as i64, *rightnum as i64) as f64)
                },
                _ => exp.runtime_error(format!("Bitwise operator can be applied only to numbers. Got {:?} and {:?}", left, right))
            )
        })
    }

    match op {
        Keyword::SOR => metatable_binop!("__bor", "+", |left, right| left | right),
        Keyword::SAND => metatable_binop!("__band", "-", |left, right| left & right),
        Keyword::TILDA => metatable_binop!("__bxor", "-", |left, right| left ^ right),
        Keyword::SHRIGHT => metatable_binop!("__bshr", "*", |left, right| left >> right),
        Keyword::SHLEFT => metatable_binop!("__bshl", "/", |left, right| left << right),
        _ => panic!("Should never happen"),
    }
}

// TODO. `or` Lazy evaluation
fn eval_boolean(
    _exp: &operators::Binop,
    op: &Keyword,
    left: types::Type,
    right: types::Type,
) -> types::Type {
    types::Type::Boolean(match op {
        Keyword::OR => left.as_bool() || right.as_bool(),
        Keyword::AND => left.as_bool() && right.as_bool(),
        _ => panic!("Should never happen"),
    })
}

fn eval_concat(
    exp: &interpreter::Eval,
    _op: &Keyword,
    left: types::Type,
    right: types::Type,
) -> types::Type {
    fn to_string(value: &types::Type) -> Option<String> {
        match_type!(value,
            types::Type::Number(num) => Some(num.to_string()),
            types::Type::String(str) => Some(str.clone()),
            _ => None
        )
    }

    if let (Some(mut leftstr), Some(rightstr)) = (to_string(&left), to_string(&right)) {
        leftstr.push_str(rightstr.as_str());
        types::Type::String(leftstr)
    } else if let types::Type::Table { ref metatable, .. } = left {
        if let Some(metamethod) = metatable.get("__concat") {
            metamethod.call(vec![&left, &right])
        } else {
            exp.runtime_error(format!(
                "{:?} metatable doesn't contain `__concat` function",
                left
            ))
        }
    } else {
        exp.runtime_error(format!(
            "Concat operator can be applied only to strings, numbers or table. Got {:?} and {:?}",
            left, right
        ))
    }
}

impl interpreter::Eval for operators::Binop {
    // ‘+’ | ‘-’ | ‘*’ | ‘/’ | ‘//’ | ‘^’ | ‘%’ |
    // ‘&’ | ‘~’ | ‘|’ | ‘>>’ | ‘<<’ | ‘..’ |
    // ‘<’ | ‘<=’ | ‘>’ | ‘>=’ | ‘==’ | ‘~=’ |
    // and | or
    // [Keyword::OR],
    // [Keyword::AND],
    // [
    //     Keyword::LESS,
    //     Keyword::LEQ,
    //     Keyword::GREATER,
    //     Keyword::GEQ,
    //     Keyword::EQ,
    //     Keyword::NEQ
    // ],
    // [Keyword::SOR],
    // [Keyword::TILDA],
    // [Keyword::SAND],
    // [Keyword::SHRIGHT, Keyword::SHLEFT],
    // [Keyword::DOT2],
    // [Keyword::PLUS, Keyword::MINUS],
    // [Keyword::MUL, Keyword::DIV, Keyword::FLOOR_DIV, Keyword::MOD],
    // [Keyword::POW]
    fn eval(&self, env: &mut utils::Shared<environment::Environment>) -> types::Type {
        let operators::Binop(op, left, right) = self;

        let left_value = left.eval(env);
        // TODO: Lazy evaluation!!!
        let right_value = right.eval(env);

        match op {
            Keyword::PLUS
            | Keyword::MINUS
            | Keyword::MUL
            | Keyword::DIV
            | Keyword::FLOORDIV
            | Keyword::MOD
            | Keyword::POW => eval_ariphmetic(self, op, left_value, right_value),
            Keyword::OR | Keyword::AND => eval_boolean(self, op, left_value, right_value),
            Keyword::LESS
            | Keyword::LEQ
            | Keyword::GREATER
            | Keyword::GEQ
            | Keyword::EQ
            | Keyword::NEQ => eval_equivalence(self, op, left_value, right_value),
            Keyword::SOR | Keyword::TILDA | Keyword::SAND | Keyword::SHRIGHT | Keyword::SHLEFT => {
                eval_bitwise(self, op, left_value, right_value)
            }
            Keyword::DOT2 => eval_concat(self, op, left_value, right_value),
            _ => panic!("Should never happen"),
        }
    }
}

impl interpreter::Eval for operators::Noop {
    fn eval(&self, _env: &mut utils::Shared<environment::Environment>) -> types::Type {
        types::Type::Nil
    }
}
