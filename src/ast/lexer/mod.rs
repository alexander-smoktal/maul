pub mod tokens;
mod tests;

use self::tokens::{get_token_table, get_operator_table, Token, TokenType, Keyword};

use std::collections::HashMap;
use std::iter::{Iterator, IntoIterator, Peekable};
use std::string::String;
use std::str::Chars;
use utils::AsExclusiveTakeWhile;

// ------------ Lexer ----------------
pub struct Lexer {
    text: String,
    token_table: HashMap<String, Keyword>,
    operator_table: HashMap<String, Keyword>,
}

impl Lexer {
    /// Create new lexer, which can be used as token iteartor
    pub fn new(input: String) -> Lexer {
        Lexer {
            text: input,
            token_table: get_token_table(),
            operator_table: get_operator_table()
        }
    }
}

// ---------- Token Iterator --------------
pub struct TokenIterator<'a> {
    lexer: &'a Lexer,
    char_iterator: Peekable<Chars<'a>>,

    // Line and column
    row: usize,
    column: usize,
}

impl<'a> TokenIterator<'a> {
    fn advance_pos(&mut self, n: usize) {
        self.column += n;
    }

    fn newline(&mut self) {
        self.row += 1;
        self.column = 0;
    }

    fn parse_next_token(&mut self) -> Option<Token> {
        if let Some(&chr) = self.char_iterator.peek() {
            if chr.is_alphabetic() || chr == '_' {
                Some(Token::new(self.parse_identifier(), self.row, self.column))
            } else if chr.is_numeric() {
                Some(Token::new(self.parse_number(), self.row, self.column))
            } else if chr == '"' {
                Some(Token::new(self.parse_string(), self.row, self.column))
            } else if chr == '\n' {
                self.char_iterator.next();
                self.newline();
                self.parse_next_token()
            } else if chr == ' ' || chr == '\t' {
                self.char_iterator.next();
                self.advance_pos(1);
                self.parse_next_token()
            } else {
                Some(Token::new(self.parse_operator(), self.row, self.column))
            }
        } else {
            None
        }
    }

    fn parse_identifier(&mut self) -> TokenType {
        let id_chars = |chr: &char| chr.is_alphanumeric();

        let id: String = self.char_iterator.take_while_exclusive(id_chars).collect();
        self.advance_pos(id.len());

        // If keyword map contains the keyword, return Token::Keyword
        // Else return a Token::Identifier
        match self.lexer.token_table.get(&id) {
            Some(keyword) => return TokenType::Keyword(keyword.clone()),
            _ => return TokenType::Id(id),
        }
    }

    fn parse_string(&mut self) -> TokenType {
        // Looking for the closing doublequote
        let string_chars = |chr: &char| chr.clone() != '"';

        // Skip starting doublequote
        self.char_iterator.next();

        let string: String = self.char_iterator.take_while_exclusive(string_chars).collect();
        self.advance_pos(string.len() + 2); // With doublequotes

        // Skip ending doublequote
        if let None = self.char_iterator.next() {
            panic!("Unmatched double quotes at {}:{}", self.row, self.column)
        }

        TokenType::String(string)
    }

    fn parse_number(&mut self) -> TokenType {
        let numeric_chars = |chr: &char| chr.is_numeric() || chr.clone() == '.';

        // Looking for the end of the number
        let number: String = self.char_iterator.take_while_exclusive(numeric_chars).collect();
        self.advance_pos(number.len());

        TokenType::Number(number)
    }

    fn parse_operator(&mut self) -> TokenType {
        // First we try longer operators then shorter, to avoid returning '>' instead of '>='
        // Lenghts are 3, 2, 1
        for n_operator in (1..4).rev() {
            let n_character_operator: String = self.char_iterator.clone().take(n_operator).collect();
            if let Some(keyword) = self.lexer.operator_table.get(&n_character_operator) {
                // Advance original iterator. Need to coolect, to power on lazy iterartor
                let _ = self.char_iterator.by_ref().take(n_operator).count();
                self.advance_pos(n_operator);

                return TokenType::Keyword(keyword.clone());
            }
        }

        panic!("Can't parse token at {}:{}", self.row, self.column)
    }
}

// ----- Iterator trait implementation for the scanner -----
impl<'a> Iterator for TokenIterator<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.parse_next_token()
    }
}

impl<'a> IntoIterator for &'a Lexer {
    type Item = Token;
    type IntoIter = TokenIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        return TokenIterator {
            lexer: self,
            char_iterator: self.text.chars().peekable(),
            row: 1,
            column: 0,
        };
    }
}
