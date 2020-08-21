use std::iter::Peekable;
use std::str::Chars;
use crate::token;
use crate::token::{Token};

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}


impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn read_identifier(&mut self, ch: char) -> String {
        let mut identifier = String::new();
        
        identifier.push(ch);
        while let Some(&ch) =  self.peek_char() {
            if is_letter(ch) {
                identifier.push(self.read_char().unwrap());
            }
            else {
                break;
            }
        }
        identifier
    }

    fn read_int(&mut self, ch: char) -> i32 {
        let mut number = String::new();

        number.push(ch);
        while let Some(&ch) = self.peek_char() {
            if ch.is_numeric() {
                number.push(self.read_char().unwrap());
            }
            else {
                break;
            }
        }
        number.parse().unwrap()
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.peek_char() {
            if ch.is_whitespace() {
                self.read_char();
            }
            else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.read_char() {
            Some('+') => Token::Plus,
            Some('=') => Token::Assign,
            Some(',') => Token::Comma,
            Some(';') => Token::Semicolon,
            Some('(') => Token::LParenthesis,
            Some(')') => Token::RParenthesis,
            Some('{') => Token::LBracket,
            Some('}') => Token::RBracket,
            Some(ch) => {
                if is_letter(ch) {
                    let identifier = self.read_identifier(ch);
                    token::lookup_identifier(identifier)
                }
                else if ch.is_numeric() {
                    Token::Int(self.read_int(ch))
                }
                else {
                    Token::Illegal
                }
            },
            None => Token::EOF
        }
    }
}
