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
        |parser, stack| -> bool {
            $(
                let result = $parse_funcs(parser, stack);

                if result {
                    debug_parser!("Or statement rule {} accepted expression {:?}. Parser state {:?}", stringify!($parse_funcs), stack.pick(), parser);
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
        |parser, stack| -> bool {
            $(if $parse_funcs(parser, stack) {
                debug_parser!("And statement rule {} accepted expression {:?}. Parser state {:?}", stringify!($parse_funcs), expression, parser);
            } else {
                debug_parser!("And statement rule {} didn't accept parser input {:?}", stringify!($parse_funcs), parser);
                return None
            }), +

            if $nandler_func(stack) {
                debug_parser!("And handling function {:?} successfully handled expression and returned {:?}", stringify!($nandler_func), expression);
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
        |parser, stack| -> bool {
            if $parse_func(parser, stack) {
                debug_parser!("Optional rule {} parsed parser input {:?}", stringify!($parse_func), result);
            } else {
                debug_parser!("Optional rule {} didn't parse parser input {:?}", stringify!($parse_func), parser);
            }

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
        |parser, stack| -> bool {
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