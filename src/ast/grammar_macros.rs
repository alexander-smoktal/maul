//trace_macros!(true);

#[macro_export]
macro_rules! or {
    [$($parse_funcs: expr),+] => {
        |parser: &mut parser::Parser, stack: &mut stack::Stack| -> bool {
            $(
                let result = $parse_funcs(parser, stack);

                if result {
                    debug_parser!("Or statement rule {} accepted expression {:?}. Parser state {:?}", stringify!($parse_funcs), stack.peek(), parser);
                    return true
                } else {
                    debug_parser!("Or statement rule {} didn't accept parser input {:?}", stringify!($parse_funcs), parser);
                }
            )+;

            debug_parser!("Or statement fails");
            false
        }
    }
}

#[macro_export]
macro_rules! and {
    [($($parse_funcs: expr),+) => $nandler_func: expr] => {
        |parser: &mut parser::Parser, stack: &mut stack::Stack| -> bool {
            let mut entered = false; // If we already accepted first token

            ($(if $parse_funcs(parser, stack) {
                entered = true;
                debug_parser!("And statement rule {} accepted expression {:?}. Parser state {:?}", stringify!($parse_funcs), stack.peek(), parser);
            } else {
                if entered {
                    panic!("TODO: Custom error message. Unexpected token: {:?}. Rule: {:?}", parser.peek(), stringify!($parse_funcs));
                }

                debug_parser!("And statement rule {} didn't accept parser input {:?}", stringify!($parse_funcs), parser);
                return false
            }), +);

            let _ = entered; // To suppress unused boolean
            $nandler_func(stack);
            debug_parser!("And handling function {:?} successfully handled expression and returned {:?}",
                stringify!($nandler_func),
                stack.peek());

            true
        }
    };
}

/// Macro has two modifications:
/// - optional!(rule) If we don't care about optional value (i.e. when optional expression handles it's head);
/// - optional!(rule, nil) If we want to know if optional rule pushed expression on stack (we handle set of expressions manually)
#[macro_export]
macro_rules! optional {
    ($parse_func:expr, nil) => {
        |parser: &mut parser::Parser, stack: &mut stack::Stack| -> bool {
            if $parse_func(parser, stack) {
                let expression = Some(stack.pop_single());
                stack.push_optional(expression);
                debug_parser!("Optional rule {} parsed parser input {:?}", stringify!($parse_func), stack.peek());
            } else {
                stack.push_optional(None);
                debug_parser!("Optional rule {} didn't parse parser input {:?}", stringify!($parse_func), parser);
            }

            true
        }
    };
    ($parse_func:expr) => {
        |parser: &mut parser::Parser, stack: &mut stack::Stack| -> bool {
            if $parse_func(parser, stack) {
                debug_parser!("Optional rule {} parsed parser input {:?}", stringify!($parse_func), stack.peek());
            } else {
                debug_parser!("Optional rule {} didn't parse parser input {:?}", stringify!($parse_func), parser);
            }

            true
        }
    }
}

#[macro_export]
macro_rules! repetition {
    ($parse_func:expr) => {
        |parser: &mut parser::Parser, stack: &mut stack::Stack| -> bool {
            let mut result = VecDeque::new();

            while $parse_func(parser, stack) {
                debug_parser!("Repeating rule {}", stringify!($parse_func));
                let single = stack.pop_single();
                result.push_back(single)
            }

            debug_parser!("Finished repetition {}", stringify!($parse_func));
            stack.push_repetition(result);
            true
        }
    }
}

#[macro_export]
macro_rules! rule {
    ($name: ident, $parse_func:expr) => {
        pub fn $name(parser: &mut parser::Parser, stack: &mut stack::Stack) -> bool {
            debug_parser!("Executing rule {}", stringify!($name));

            $parse_func(parser, stack)
        }
    };
}

#[macro_export]
macro_rules! terminal {
    ($keyword: expr) => {
        |parser: &mut parser::Parser, stack: &mut stack::Stack| -> bool {
            if let Some(token) = parser.peek().cloned() {
                match token.keyword() {
                    Some(keyword) => {
                        if keyword == $keyword {
                            parser.shift();
                            debug_parser!("Accepted keyword {:?}", keyword);
                            stack.push_single(Box::new(::ast::expressions::Terminal(keyword)));
                            true
                        } else {
                            false
                        }
                    }
                    _ => false
                }
            } else {
                false
            }
        }
    }
}