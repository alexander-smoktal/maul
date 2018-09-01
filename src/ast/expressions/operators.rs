use std::collections::HashSet;

use super::*;
use ast::parser;
use ast::stack;
use ast::parser::rules;
use ast::lexer::tokens;

// binop ::=  ‘+’ | ‘-’ | ‘*’ | ‘/’ | ‘//’ | ‘^’ | ‘%’ |
//        ‘&’ | ‘~’ | ‘|’ | ‘>>’ | ‘<<’ | ‘..’ |
//        ‘<’ | ‘<=’ | ‘>’ | ‘>=’ | ‘==’ | ‘~=’ |
//        and | or
#[derive(Debug)]
pub struct Binop(
    pub tokens::Keyword,
    pub Box<expression::Expression>,
    pub Box<expression::Expression>
);
impl expression::Expression for Binop {}

impl Binop {
    pub fn rule(parser: &mut parser::Parser, stack: &mut stack::Stack) -> bool {
        let terminals: HashSet<tokens::Keyword> = 
            vec![tokens::Keyword::PLUS, tokens::Keyword::MINUS, tokens::Keyword::MUL, tokens::Keyword::DIV,
            tokens::Keyword::POW, tokens::Keyword::MOD, tokens::Keyword::AND, tokens::Keyword::TILDA,
            tokens::Keyword::OR, tokens::Keyword::SHRIGHT, tokens::Keyword::SHLEFT, tokens::Keyword::DOT2, tokens::Keyword::LESS,
            tokens::Keyword::LEQ, tokens::Keyword::GREATER, tokens::Keyword::GEQ, tokens::Keyword::EQ, tokens::Keyword::NEQ,
            tokens::Keyword::AND, tokens::Keyword::OR].into_iter().collect();

        if let Some(token) = parser.peek().cloned() {
            if let Some(keyword) = token.keyword() {

                if !terminals.contains(&keyword) {
                    return false
                }

                parser.shift();

                if rules::exp(parser, stack) {
                    let (expression_right, expression_left) = stack_unpack!(stack, single, single);

                    stack.push_single(Box::new(Binop(keyword, expression_left, expression_right)));
                    return true
                } else {
                    panic!(format!("Expecter expressiion after binary operator, got {:?}", parser.peek()))
                }
            }
        }

        false
    }
}

// unop ::= ‘-’ | not | ‘#’ | ‘~’
#[derive(Debug)]
pub struct Unop(pub tokens::Keyword, pub Box<expression::Expression>);
impl expression::Expression for Unop {}

impl Unop {
    pub fn rule(parser: &mut parser::Parser, stack: &mut stack::Stack) -> bool {
        let terminals: HashSet<tokens::Keyword> = 
            vec![tokens::Keyword::MINUS, tokens::Keyword::NOT,
            tokens::Keyword::HASH, tokens::Keyword::TILDA].into_iter().collect();

        if let Some(token) = parser.peek().cloned() {
            if let Some(keyword) = token.keyword() {

                if !terminals.contains(&keyword) {
                    return false
                }
                
                parser.shift();

                if rules::exp_prefix(parser, stack) {
                    let expression = stack.pop_single();

                    stack.push_single(Box::new(Unop(keyword, expression)));
                    return true
                } else {
                    panic!(format!("Expecter expressiion after unary operator, got {:?}", parser.peek()))
                }
            }
        }

        false
    }
}

#[derive(Debug)]
pub struct Noop;
impl expression::Expression for Noop {}

impl Noop {
    make_keyword_rule![semi, (tokens::Keyword::SEMICOLONS, Noop)];
}
