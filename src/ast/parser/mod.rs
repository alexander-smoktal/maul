#[macro_use]
pub mod parse_macros;

use super::lexer::{Lexer, tokens};

type Position = usize;

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
