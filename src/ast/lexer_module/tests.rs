#![cfg(test)]

use super::lexer;
use super::tokens::*;

#[test]
fn test_empty_lexer() {
    let lex = lexer::Lexer::new("".to_owned());

    for _ in &lex {
        unreachable!();
    }
}


#[test]
fn test_keywords() {
    let lex = lexer::Lexer::new("and Hello or World while".to_owned());
    let mut iter = lex.into_iter();

    assert_eq!(iter.next(), Some(Token::new(TokenType::Keyword(Keyword::AND), 1, 4)));
    assert_eq!(iter.next(), Some(Token::new(TokenType::Id("Hello".as_bytes()), 1, 10)));
    assert_eq!(iter.next(), Some(Token::new(TokenType::Keyword(Keyword::OR), 1, 13)));
    assert_eq!(iter.next(), Some(Token::new(TokenType::Id("World".as_bytes()), 1, 19)));
    assert_eq!(iter.next(), Some(Token::new(TokenType::Keyword(Keyword::WHILE), 1, 25)));
    assert_eq!(iter.next(), None);
}


#[test]
fn test_strings() {
    let lex = lexer::Lexer::new("\"Hello\" \"world\"\"!\"".to_owned());
    let mut iter = lex.into_iter();

    assert_eq!(iter.next(), Some(Token::new(TokenType::String("Hello".as_bytes()), 1, 8)));
    assert_eq!(iter.next(), Some(Token::new(TokenType::String("world".as_bytes()), 1, 16)));
    assert_eq!(iter.next(), Some(Token::new(TokenType::String("!".as_bytes()), 1, 19)));
    assert_eq!(iter.next(), None);
}

#[should_panic]
#[test]
fn test_invalid_strings() {
    let lex = lexer::Lexer::new("\"Hello".to_owned());
    let mut iter = lex.into_iter();

    assert_eq!(iter.next(), Some(Token::new(TokenType::String("Hello".as_bytes()), 1, 8)));
}

#[test]
fn test_numbers() {
    let lex = lexer::Lexer::new("3 43 42.42 777".to_owned());
    let mut iter = lex.into_iter();

    assert_eq!(iter.next(), Some(Token::new(TokenType::Number("3".as_bytes()), 1, 2)));
    assert_eq!(iter.next(), Some(Token::new(TokenType::Number("43".as_bytes()), 1, 5)));
    assert_eq!(iter.next(), Some(Token::new(TokenType::Number("42.42".as_bytes()), 1, 11)));
    assert_eq!(iter.next(), Some(Token::new(TokenType::Number("777".as_bytes()), 1, 15)));
    assert_eq!(iter.next(), None);
}
