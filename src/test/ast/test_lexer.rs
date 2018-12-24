#![cfg(test)]

use crate::ast::lexer::tokens::*;
use crate::ast::parser;

struct ParseWrapper {
    parser: parser::Parser,
}

impl ParseWrapper {
    pub fn new(source: &str) -> Self {
        ParseWrapper {
            parser: parser::Parser::new(source.to_string()),
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        if let Some(token) = self.parser.peek().cloned() {
            self.parser.shift();
            Some(token)
        } else {
            None
        }
    }
}

#[test]
fn test_empty_lexer() {
    let mut parser = ParseWrapper::new("");

    if let Some(ref _element) = parser.next() {
        unreachable!()
    }
}

#[test]
fn test_keywords() {
    let mut parser = ParseWrapper::new("and Hello or World while");

    assert_eq!(
        parser.next(),
        Some(Token::new(TokenType::Keyword(Keyword::AND), 1, 3))
    );
    assert_eq!(
        parser.next(),
        Some(Token::new(TokenType::Id(String::from("Hello")), 1, 9))
    );
    assert_eq!(
        parser.next(),
        Some(Token::new(TokenType::Keyword(Keyword::OR), 1, 12))
    );
    assert_eq!(
        parser.next(),
        Some(Token::new(TokenType::Id(String::from("World")), 1, 18))
    );
    assert_eq!(
        parser.next(),
        Some(Token::new(TokenType::Keyword(Keyword::WHILE), 1, 24))
    );
    assert_eq!(parser.next(), None);
}

#[test]
fn test_strings() {
    let mut parser = ParseWrapper::new(r#""Hello" "world""!""#);

    assert_eq!(
        parser.next(),
        Some(Token::new(TokenType::String(String::from("Hello")), 1, 7))
    );
    assert_eq!(
        parser.next(),
        Some(Token::new(TokenType::String(String::from("world")), 1, 15))
    );
    assert_eq!(
        parser.next(),
        Some(Token::new(TokenType::String(String::from("!")), 1, 18))
    );
    assert_eq!(parser.next(), None);
}

#[test]
#[should_panic]
fn test_invalid_strings() {
    let mut parser = ParseWrapper::new(r#""Hello"#);

    assert_eq!(
        parser.next(),
        Some(Token::new(TokenType::String(String::from("Hello")), 1, 8))
    );
}

#[test]
fn test_numbers() {
    let mut parser = ParseWrapper::new("3 43 42.42 777");

    assert_eq!(
        parser.next(),
        Some(Token::new(TokenType::Number(3f64), 1, 1))
    );
    assert_eq!(
        parser.next(),
        Some(Token::new(TokenType::Number(43f64), 1, 4))
    );
    assert_eq!(
        parser.next(),
        Some(Token::new(TokenType::Number(42.42f64), 1, 10))
    );
    assert_eq!(
        parser.next(),
        Some(Token::new(TokenType::Number(777f64), 1, 14))
    );
    assert_eq!(parser.next(), None);
}
