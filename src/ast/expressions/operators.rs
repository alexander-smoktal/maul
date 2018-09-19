use std::collections::{HashMap, HashSet};

use ast::expressions;
use ast::lexer::tokens::{self, Keyword};
use ast::parser;
use ast::rules;
use ast::stack;

const DEBUG: bool = false;

macro_rules! make_op_table {
    ($lrec_prec: ident, $([$($lrec: expr), +]), +) => {
        let mut $lrec_prec = HashMap::new();

        let mut prec: usize = 0;
        $(
            $(
                $lrec_prec.insert($lrec, prec);
            )+
            prec = prec + 1;
            let _compiler_friend = prec;
        )+
    };
}

// binop ::=  ‘+’ | ‘-’ | ‘*’ | ‘/’ | ‘//’ | ‘^’ | ‘%’ |
//        ‘&’ | ‘~’ | ‘|’ | ‘>>’ | ‘<<’ | ‘..’ |
//        ‘<’ | ‘<=’ | ‘>’ | ‘>=’ | ‘==’ | ‘~=’ |
//        and | or
#[derive(Debug)]
pub struct Binop(
    pub Keyword,
    pub Box<expressions::Expression>,
    pub Box<expressions::Expression>,
);
impl expressions::Expression for Binop {}

impl Binop {
    // parse_expression () return parse_expression_1 (parse_primary (), 0)
    pub fn rule(parser: &mut parser::Parser, stack: &mut stack::Stack) -> bool {
        if !rules::exp_prefix(parser, stack) {
            return false;
        }

        Binop::prec_rule(parser, stack, 0);
        true
    }

    // https://en.wikipedia.org/wiki/Operator-precedence_parser
    // parse_expression_1 (lhs, min_precedence)
    //     lookahead := peek next token
    //     while lookahead is a binary operator whose precedence is >= min_precedence
    //         op := lookahead
    //         advance to next token
    //         rhs := parse_primary ()
    //         lookahead := peek next token
    //         while lookahead is a binary operator whose precedence is greater
    //                  than op's, or a right-associative operator
    //                  whose precedence is equal to op's
    //             rhs := parse_expression_1 (rhs, lookahead's precedence)
    //             lookahead := peek next token
    //         lhs := the result of applying op with operands lhs and rhs
    //     return lhs
    pub fn prec_rule(
        parser: &mut parser::Parser,
        stack: &mut stack::Stack,
        precedence: usize,
    ) {
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
        make_op_table!(
            ops_prec_table,
            [Keyword::OR],
            [Keyword::AND],
            [
                Keyword::LESS,
                Keyword::LEQ,
                Keyword::GREATER,
                Keyword::GEQ,
                Keyword::EQ,
                Keyword::NEQ
            ],
            [Keyword::SOR],
            [Keyword::TILDA],
            [Keyword::SAND],
            [Keyword::SHRIGHT, Keyword::SHLEFT],
            [Keyword::DOT2],
            [Keyword::PLUS, Keyword::MINUS],
            [Keyword::MUL, Keyword::DIV, Keyword::MOD, Keyword::FLOORDIV],
            [Keyword::POW]
        );

        debug_parser!(
            "Entered precedence rule. Left: {:?}. Precedence: {}",
            stack.peek(),
            precedence
        );

        // lookahead := peek next token
        // while lookahead is a binary operator whose precedence is >= min_precedence
        while let Some(op) = parser.peek().and_then(|token| token.keyword()) {
            if !ops_prec_table.contains_key(&op) || ops_prec_table[&op] < precedence {
                return;
            }

            // op := lookahead
            // advance to next token
            parser.shift();

            // rhs := parse_primary ()
            if !rules::exp_prefix(parser, stack) {
                panic!(format!(
                    "Expecter expressiion after binary operator, got {:?}",
                    parser.peek()
                ))
            }

            // lookahead := peek next token
            while let Some(op2) = parser.peek().and_then(|token| token.keyword()) {
                // while lookahead is a binary operator whose precedence is greater
                // than op's, or a right-associative operator
                // whose precedence is equal to op's
                if !ops_prec_table.contains_key(&op2) || ops_prec_table[&op2] <= ops_prec_table[&op]
                {
                    break;
                }
                // rhs := parse_expression_1 (rhs, lookahead's precedence)
                // lookahead := peek next token
                Binop::prec_rule(parser, stack, ops_prec_table[&op2]);
            }
            let (right_expression, left_expression) = stack_unpack!(stack, single, single);

            // lhs := the result of applying op with operands lhs and rhs
            stack.push_single(Box::new(Binop(op, left_expression, right_expression)));
        }
    }
}

// unop ::= ‘-’ | not | ‘#’ | ‘~’
#[derive(Debug)]
pub struct Unop(pub Keyword, pub Box<expressions::Expression>);
impl expressions::Expression for Unop {}

impl Unop {
    pub fn rule(parser: &mut parser::Parser, stack: &mut stack::Stack) -> bool {
        let terminals: HashSet<Keyword> =
            vec![Keyword::MINUS, Keyword::NOT, Keyword::HASH, Keyword::TILDA]
                .into_iter()
                .collect();

        if let Some(token) = parser.peek().cloned() {
            if let Some(keyword) = token.keyword() {
                if !terminals.contains(&keyword) {
                    return false;
                }

                parser.shift();

                if rules::exp_prefix(parser, stack) {
                    let expression = stack.pop_single();

                    stack.push_single(Box::new(Unop(keyword, expression)));
                    return true;
                } else {
                    panic!(format!(
                        "Expecter expressiion after unary operator, got {:?}",
                        parser.peek()
                    ))
                }
            }
        }

        false
    }
}

#[derive(Debug)]
pub struct Noop;
impl expressions::Expression for Noop {}

impl Noop {
    make_keyword_rule![semi, (Keyword::SEMICOLONS, Noop)];
}
