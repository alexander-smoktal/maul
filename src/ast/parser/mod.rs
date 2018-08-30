#[macro_use]
pub mod parse_macros;
pub mod rules;

use super::lexer::{Lexer, tokens};

#[derive(Debug)]
pub struct Parser {
    /// Lexer
    lexer: Lexer,
    /// Active token
    lookahead_token: Option<tokens::Token>,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        Parser {
            lexer: Lexer::new(input),
            lookahead_token: None
        }
    }

    // TODO: Store borrowed positions and trim vector for all prefix token we can't access anymore
    pub fn peek(&mut self) -> Option<&tokens::Token> {
        if let Some(ref token) = self.lookahead_token {
            Some(token)
        } else {
            if let Some(token) = self.lexer.next() {
                self.lookahead_token = Some(token);
                self.lookahead_token.as_ref()
            } else {
                None
            }
        }
    }

    pub fn shift(&mut self) {
        self.lookahead_token = None
    }
}
