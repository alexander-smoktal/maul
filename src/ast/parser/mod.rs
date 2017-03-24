mod tests;

use super::lexer::Lexer;

#[derive(Clone)]
pub struct Parser {
    lexer: Lexer,
}

/// I'll try to make this recursive descendant parser, buy hey... nobody is perfect
impl Parser {
    pub fn new(input: String) -> Parser {
        Parser { lexer: Lexer::new(input) }
    }
}

impl Iterator for Parser {
    type Item = <Lexer as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.next()
    }
}