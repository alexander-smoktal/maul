#[macro_use]
pub mod parse_macros;

use super::lexer::{Lexer, tokens};
use super::expressions;

type Position = usize;

pub fn keyword(keyword: tokens::Keyword) -> impl FnMut(&mut Parser) -> Option<Box<expressions::expression::Expression>> {
    move |ref mut parser| {
        let parser_pos = parser.position();

        parser
        .next()
        .cloned()
        .and_then(|token|
            if token.keyword() == Some(keyword.clone()) {
                Some(Box::new(expressions::common::Noop) as Box<expressions::expression::Expression>)
            } else {
                parser.rollback(parser_pos);
                None
            })
    }
}

#[derive(Debug)]
pub struct Parser {
    /// Lexer
    lexer: Lexer,
    /// Vector of already read tokens
    read_input: Vec<tokens::Token>,
    /// Input position
    position: usize
}

impl Parser {
    pub fn new(input: String) -> Parser {
        Parser {
            lexer: Lexer::new(input),
            read_input: vec![],
            position: 0
        }
    }

    // TODO: Store borrowed positions and trim vector for all prefix token we can't access anymore
    pub fn position(&self) -> Position {
        self.position
    }

    pub fn next(&mut self) -> Option<&tokens::Token> {
        // First we check if we have not enough tokens. If so, we try to gen one more
        if self.position >= self.read_input.len() {
            if let Some(token) = self.lexer.next() {
                self.read_input.push(token);
            // If not tokens, return None
            } else {
                return None
            }
        }

        // We ither had enough tokens before, or pushed new tokens on top of read tokens vec
        self.position += 1;
        Some(&self.read_input[self.position - 1])
    }

    pub fn rollback(&mut self, position: Position) {
        self.position = position
    }
}
