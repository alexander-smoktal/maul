use ast::expressions::operators;
use ast::lexer::tokens::Keyword;

use interpreter::{self, environment, types};

impl interpreter::Eval for operators::Unop {
    // unop ::= ‘-’ | not | ‘#’ | ‘~’
    // Keyword::MINUS, Keyword::NOT, Keyword::HASH, Keyword::TILDA
    fn eval(&self, env: &mut environment::Environment) -> types::Type {
        let value = self.1.eval(env);

        // Keyword
        match self.0 {
            Keyword::MINUS => {
                if let types::Type::Number(number) = value {
                    types::Type::Number(-number)
                } else {
                    self.runtime_error(format!("Can't negate {:?} value", value));
                }
            }
            Keyword::NOT => {
                match value {
                    types::Type::Nil => types::Type::Boolean(true),
                    types::Type::Boolean(false) => types::Type::Boolean(true),
                    _ => types::Type::Boolean(false)
                }
            },
            Keyword::HASH => {
                match value {
                    types::Type::String(string) => types::Type::Number(string.as_bytes().len() as f64),
                    types::Type::Table { border, ref metatable, .. } => {
                        if let Some(metamethod) = metatable.get("__len") {
                            metamethod.call(vec![&value])
                        } else {
                            types::Type::Number(border as f64)
                        }
                    },
                    _ => {
                        self.runtime_error(format!("Can't get length of {:?} value", value));
                    }
                }
            },
            Keyword::TILDA => {
                if let types::Type::Number(number) = value {
                    types::Type::Number(!(number as i64) as f64)
                } else {
                    self.runtime_error(format!("Can't apply bitwise not to {:?} value", value));
                }
            },
            _ => panic!("Should never happen")
        }
    }
}



pub fn eval_ariphmetic(exp: &interpreter::Eval, op: &Keyword, left: types::Type, right: types::Type) -> types::Type {
    // Function to convert value for arithmetic operation
    let normalize = |value, op| -> f64 {
        match value {
            types::Type::Number(number) => number,
            types::Type::String(string) => {
                if let Ok(number) = string.parse::<f64>() {
                    number
                } else {
                    exp.runtime_error(format!("Can't convert string {:?} to apply {} operator", string, op))
                }
            },
            _ => exp.runtime_error(format!("Can't apply {} operator to {:?} value", op, value))
        }
    };

    macro_rules! metatable_binop {
        ($mt_key: tt, $op: tt, $function: expr) => {
            if let types::Type::Table { ref metatable, .. } = left {
                    if let Some(metamethod) = metatable.get($mt_key) {
                        return metamethod.call(vec![&left, &right])
                    }
                }

            return types::Type::Number($function(normalize(left, $op), normalize(right, $op)))
        }
    }

    match op {
        Keyword::PLUS => {
            metatable_binop!("__add", "+", |left, right| { left + right });
        },
        Keyword::MINUS => {
            metatable_binop!("__sub", "-", |left, right| { left - right });
        },
        Keyword::MUL => {
            metatable_binop!("__mul", "*", |left, right| { left * right });
        },
        Keyword::DIV => {
            metatable_binop!("__div", "/", |left, right| { left / right });
        },
        Keyword::FLOORDIV => {
            metatable_binop!("__idiv", "//", |left: f64, right: f64| { (left / right).floor() });
        },
        Keyword::MOD => {
            metatable_binop!("__mod", "%", |left: f64, right: f64| { left % right });
        },
        Keyword::POW => {
            metatable_binop!("__pow", "^", |left: f64, right: f64| { left.powf(right) });
        },
        _ =>  panic!("Should never happen")
    }
}

pub fn eval_equivalence(exp: &interpreter::Eval, op: &Keyword, left: types::Type, right: types::Type) -> types::Type {
    fn not(value: types::Type) -> types::Type {
        match value {
            types::Type::Nil => types::Type::Boolean(true),
            types::Type::Boolean(false) => types::Type::Boolean(true),
            _ => types::Type::Boolean(false)
        }
    }

    match (&left, &right) {
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
        }
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
            match op {
                Keyword::LESS => {
                    if let Some(metamethod) = metatable.get("__lt") {
                        metamethod.call(vec![&left, &right])
                    } else {
                        exp.runtime_error(format!("Can't compare values {:?} and {:?} with {} operator", left, right, "<"))
                    }
                }
                Keyword::LEQ => {
                    if let Some(metamethod) = metatable.get("__le") {
                        metamethod.call(vec![&left, &right])
                    } else {
                        exp.runtime_error(format!("Can't compare values {:?} and {:?} with {} operator", left, right, "<="))
                    }
                }
                Keyword::GREATER => {
                    if let Some(metamethod) = metatable.get("__le") {
                        not(metamethod.call(vec![&left, &right]))
                    } else {
                        exp.runtime_error(format!("Can't compare values {:?} and {:?} with {} operator", left, right, "<"))
                    }
                }
                Keyword::GEQ => {
                    if let Some(metamethod) = metatable.get("__lt") {
                        not(metamethod.call(vec![&left, &right]))
                    } else {
                        exp.runtime_error(format!("Can't compare values {:?} and {:?} with {} operator", left, right, "<"))
                    }
                }
                Keyword::EQ => {
                    if let Some(metamethod) = metatable.get("__eq") {
                        metamethod.call(vec![&left, &right])
                    } else {
                        exp.runtime_error(format!("Can't compare values {:?} and {:?} with {} operator", left, right, "<"))
                    }
                }
                Keyword::NEQ => {
                    if let Some(metamethod) = metatable.get("__eq") {
                        not(metamethod.call(vec![&left, &right]))
                    } else {
                        exp.runtime_error(format!("Can't compare values {:?} and {:?} with {} operator", left, right, "<"))
                    }
                }
                _ => panic!("Should never happen")
            }
        }
        _ => unimplemented!()
    }
}

pub fn eval_bitwise(exp: &operators::Binop, op: &Keyword, left: types::Type, right: types::Type) -> types::Type {
    types::Type::Nil
}

pub fn eval_boolean(exp: &operators::Binop, op: &Keyword, left: types::Type, right: types::Type) -> types::Type {
    types::Type::Nil
}

pub fn eval_concat(exp: &operators::Binop, op: &Keyword, left: types::Type, right: types::Type) -> types::Type {
    types::Type::Nil
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
    fn eval(&self, env: &mut environment::Environment) -> types::Type {
        let operators::Binop(op, left, right) = self;

        let left_value = left.eval(env);
        let right_value = right.eval(env);

        match op {
            Keyword::PLUS | Keyword::MINUS | Keyword::MUL | Keyword::DIV | Keyword::FLOORDIV | Keyword::MOD | Keyword::POW => eval_ariphmetic(self, op, left_value, right_value),
            Keyword::OR | Keyword::AND => eval_boolean(self, op, left_value, right_value),
            Keyword::LESS | Keyword::LEQ | Keyword::GREATER | Keyword::GEQ | Keyword::EQ | Keyword::NEQ => eval_equivalence(self, op, left_value, right_value),
            Keyword::SOR | Keyword::TILDA | Keyword::SAND | Keyword::SHRIGHT | Keyword::SHLEFT => eval_bitwise(self, op, left_value, right_value),
            Keyword::DOT2 => eval_concat(self, op, left_value, right_value),
            _ => panic!("Should never happen")
        }
    }
}

impl interpreter::Eval for operators::Noop {
    fn eval(&self, _env: &mut environment::Environment) -> types::Type {
        types::Type::Nil
    }
}