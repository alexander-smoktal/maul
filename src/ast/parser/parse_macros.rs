//trace_macros!(true);

#[macro_export]
macro_rules! debug_parser {
    ($($params: expr), +) => {
        if DEBUG {
            println!($($params,) +);
        }
    };
}

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
            ($(if $parse_funcs(parser, stack) {
                debug_parser!("And statement rule {} accepted expression {:?}. Parser state {:?}", stringify!($parse_funcs), stack.peek(), parser);
            } else {
                debug_parser!("And statement rule {} didn't accept parser input {:?}", stringify!($parse_funcs), parser);
                return false
            }), +);

            if $nandler_func(stack) {
                debug_parser!("And handling function {:?} successfully handled expression and returned {:?}",
                    stringify!($nandler_func),
                    stack.peek());
                true
            } else {
                debug_parser!("And handling function {:?} failed to process expressions", stringify!($nandler_func));
                false
            }
        }
    };
}

#[macro_export]
macro_rules! optional {
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
            let mut result = vec![];

            while $parse_func(parser, stack) {
                let single = stack.pop_single();
                result.push(single)
            }

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
                if token.keyword() == Some($keyword) {
                    parser.shift();
                    stack.push_single(Box::new(operators::Noop));

                    true
                } else {
                    false
                }
            } else {
                false
            }
        }
    }
}