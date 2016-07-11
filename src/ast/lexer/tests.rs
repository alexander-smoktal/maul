#![cfg(test)]

use super::tokens::*;
use super::*;

#[test]
fn test_empty_lexer() {
    let lex = Lexer::new("".to_owned());

    for _ in &lex {
        unreachable!();
    }
}


#[test]
fn test_keywords() {
    let lex = Lexer::new("and Hello or World while".to_owned());
    let mut iter = lex.into_iter();

    assert_eq!(iter.next(),
               Some(Token::new(TokenType::Keyword(Keyword::AND), 1, 3)));
    assert_eq!(iter.next(),
               Some(Token::new(TokenType::Id(String::from("Hello")), 1, 9)));
    assert_eq!(iter.next(),
               Some(Token::new(TokenType::Keyword(Keyword::OR), 1, 12)));
    assert_eq!(iter.next(),
               Some(Token::new(TokenType::Id(String::from("World")), 1, 18)));
    assert_eq!(iter.next(),
               Some(Token::new(TokenType::Keyword(Keyword::WHILE), 1, 24)));
    assert_eq!(iter.next(), None);
}


#[test]
fn test_strings() {
    let lex = Lexer::new(r#""Hello" "world""!""#.to_owned());
    let mut iter = lex.into_iter();

    assert_eq!(iter.next(),
               Some(Token::new(TokenType::String(String::from("Hello")), 1, 7)));
    assert_eq!(iter.next(),
               Some(Token::new(TokenType::String(String::from("world")), 1, 15)));
    assert_eq!(iter.next(),
               Some(Token::new(TokenType::String(String::from("!")), 1, 18)));
    assert_eq!(iter.next(), None);
}

#[should_panic]
#[test]
fn test_invalid_strings() {
    let lex = Lexer::new(r#""Hello"#.to_owned());
    let mut iter = lex.into_iter();

    assert_eq!(iter.next(),
               Some(Token::new(TokenType::String(String::from("Hello")), 1, 8)));
}

#[test]
fn test_numbers() {
    let lex = Lexer::new("3 43 42.42 777".to_owned());
    let mut iter = lex.into_iter();

    assert_eq!(iter.next(),
               Some(Token::new(TokenType::Number(String::from("3")), 1, 1)));
    assert_eq!(iter.next(),
               Some(Token::new(TokenType::Number(String::from("43")), 1, 4)));
    assert_eq!(iter.next(),
               Some(Token::new(TokenType::Number(String::from("42.42")), 1, 10)));
    assert_eq!(iter.next(),
               Some(Token::new(TokenType::Number(String::from("777")), 1, 14)));
    assert_eq!(iter.next(), None);
}
