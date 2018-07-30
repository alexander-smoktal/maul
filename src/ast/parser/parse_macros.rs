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
        |parser: &mut parser::Parser| -> Option<Box<expression::Expression>> {
            $(
                let parser_pos = parser.position();
                let result = $parse_funcs(parser);

                if result.is_some() {
                    debug_parser!("Or statement rule {} accepted expression {:?}. Parser state {:?}", stringify!($parse_funcs), result, parser);
                    return result
                } else {
                    parser.rollback(parser_pos);
                    debug_parser!("Or statement rule {} didn't accept parser input {:?}", stringify!($parse_funcs), parser);
                }
            )+;

            debug_parser!("Or statement fails");
            None
        }
    }
}

#[macro_export]
macro_rules! and {
    [($($parse_funcs: expr),+) => $nandler_func: expr] => {
        |parser: &mut parser::Parser| -> Option<Box<expression::Expression>> {
            let parser_pos = parser.position();

            let results = ($(match $parse_funcs(parser) {
                Some(expression) => {
                    debug_parser!("And statement rule {} accepted expression {:?}. Parser state {:?}", stringify!($parse_funcs), expression, parser);
                    expression
                }
                _ => {
                    debug_parser!("And statement rule {} didn't accept parser input {:?}", stringify!($parse_funcs), parser);
                    parser.rollback(parser_pos);
                    return None
                }
            }), +);

            match std::ops::Fn::call(&$nandler_func, results) {
                expression @ Some(_) => {
                    debug_parser!("And handling function successfully handled expression and returned {:?}", expression);
                    expression
                }
                _ => {
                    debug_parser!("And handling function failed to process expressions");
                    parser.rollback(parser_pos);
                    return None
                }
            }
        }
    };
}

#[macro_export]
macro_rules! rule {
    ($name: ident, $parse_func:expr) => {
        fn $name(parser: &mut parser::Parser) -> Option<Box<expression::Expression>> {
            debug_parser!("Executing rule {}", stringify!($name));

            $parse_func(parser)
        }
    };
}