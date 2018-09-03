use super::lexer::{Lexer, tokens};

const DEBUG: bool = false;

#[derive(Debug)]
pub enum Result {
    Ok,
    Error(String)
}

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

    pub fn peek(&mut self) -> Option<&tokens::Token> {
        let result = if let Some(ref token) = self.lookahead_token {
            Some(token)
        } else {
            if let Some(token) = self.lexer.next() {
                self.lookahead_token = Some(token);
                self.lookahead_token.as_ref()
            } else {
                None
            }
        };

        debug_parser!("Parser peek {:?}", result);
        result
    }

    /// Function to shift parset. Must be called only by functions, which consume token
    pub fn shift(&mut self) {
        debug_parser!("Parser shift {:?}", self.lookahead_token);
        self.lookahead_token = None
    }
}
