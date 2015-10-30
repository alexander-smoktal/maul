use super::tokens::{get_token_table, Token, TokenType, Keyword};

use std::collections::HashMap;
use std::iter::{Iterator, IntoIterator};
use std::string::String;
use std::str;

// ------------ Lexer ----------------
pub struct Lexer {
    text: String,
    token_table: HashMap<&'static str, Keyword>
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer { text: input, token_table: get_token_table() }
    }
}

// ---------- Token Iterator --------------
pub struct TokenIterator<'a> {
    lexer: &'a Lexer,
    code: &'a[u8],
    pos: usize,

    // Line and column
    row: usize,
    column: usize
}

impl<'a> TokenIterator<'a> {
    fn advance_pos(&mut self, n: usize) {
        self.pos += n;
        self.column += n;
    }

    fn newline(&mut self) {
        self.row += 1;
        self.column = 0;
    }

    fn parse_next_token(&mut self) -> Option<Token<'a>> {

        while self.pos < self.code.len() {
            let chr = self.code[self.pos] as char;

            if chr.is_alphabetic() {
                return Some(Token::new(self.parse_identifier(), self.row, self.column))
            } else if chr.is_numeric() {
                return Some(Token::new(self.parse_number(), self.row, self.column))
            } else if chr == '\n' {
                self.newline()
            } else if chr == '"' {
                return Some(Token::new(self.parse_string(), self.row, self.column))
            } else if chr == ' ' || chr == '\t' || chr == '\n' {
                self.advance_pos(1);
            } else {
                return Some(Token::new(self.parse_operator(), self.row, self.column))
            }
        }

        return None;
    }

    fn parse_identifier(&mut self) -> TokenType<'a> {
        let allowed_chars = |a: &u8| {
            let chr = *a as char;

            return !(chr.is_alphabetic() || chr.is_numeric())
        };

        let (_, mut slice) = self.code.split_at(self.pos);

        // Get all allowed symbols for the identifier
        if let Some(index) = slice.iter().position(allowed_chars) {
            self.advance_pos(index);

            let (slice_tmp, _) = slice.split_at(index);
            slice = slice_tmp;
        } else {
            self.advance_pos(slice.len())
        }

        match str::from_utf8(slice) {
            // If keyword map contains the keyword, return Token::Keyword
            // Else return a Token::Identifier
            Ok(key) => match self.lexer.token_table.get(key) {
                Some(keyword) => return TokenType::Keyword(keyword.clone()),
                _ => return TokenType::Id(slice)
            },
            Err(why) => panic!("{:?}", why)
        }
    }

    fn parse_string(&mut self) -> TokenType<'a> {
        let string_chars = |a: &u8| {
            let chr = *a as char;

            return chr == '"'
        };

        self.advance_pos(1);
        let (_, slice) = self.code.split_at(self.pos);

        // Looking for the closing doublequote
        if let Some(index) = slice.iter().position(string_chars) {
            self.advance_pos(index);
            let (slice, _) = slice.split_at(index);

            self.advance_pos(1);
            return TokenType::String(slice);
        }
        panic!("Unmatched double quotes at {}:{}", self.row, self.column)
    }

    fn parse_number(&mut self) -> TokenType<'a> {
        let numeric_chars = |a: &u8| {
            let chr = *a as char;

            return !(chr.is_numeric() || chr == '.')
        };

        let (_, mut slice) = self.code.split_at(self.pos);

        // Looking for the end of the number
        if let Some(index) = slice.iter().position(numeric_chars) {
            self.advance_pos(index);

            let (slice_tmp, _) = slice.split_at(index);
            slice = slice_tmp;
        } else {
            // If number is in the end of the code (impossible actually)
            self.advance_pos(slice.len())
        }

        return TokenType::Number(slice)
    }

    fn parse_operator(&mut self) -> TokenType<'a> {
        return TokenType::Keyword(Keyword::AND);
    }
}

// ----- Iterator traits implementation -----
impl<'a> Iterator for TokenIterator<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        return self.parse_next_token()
    }
}

impl<'a> IntoIterator for &'a Lexer {
    type Item = Token<'a>;
    type IntoIter = TokenIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        return TokenIterator { lexer: self, code: self.text.as_bytes(), pos: 0, row: 1, column: 1 };
    }
}
