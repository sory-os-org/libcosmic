//! Tokenizer CSS — analyse lexicale des feuilles de style.
//!
//! Produit une séquence de tokens à partir d'une chaîne CSS brute.
//! Supporte la spécification CSS Syntax Level 3 (tokens de base).

use std::iter::Peekable;
use std::str::Chars;

/// Token CSS.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    AtKeyword(String),
    Hash(String),
    String(String),
    Number(f64),
    Percentage(f64),
    Dimension(f64, String),
    Colon,
    Semicolon,
    Comma,
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Dot,
    HashToken,
    Whitespace,
    Comment,
    Delim(char),
    Eof,
}

/// Tokeniseur CSS.
pub struct Tokenizer<'a> {
    input: Peekable<Chars<'a>>,
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            pos: 0,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.input.next();
        if c.is_some() {
            self.pos += 1;
        }
        c
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn consume_whitespace(&mut self) {
        while let Some(&c) = self.peek() {
            if c.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }
    }

    fn consume_comment(&mut self) {
        // Assumes we've already consumed "/*"
        loop {
            match self.next_char() {
                Some('*') if self.peek() == Some(&'/') => {
                    self.next_char();
                    return;
                }
                Some(_) => continue,
                None => return,
            }
        }
    }

    fn consume_ident(&mut self, first: char) -> String {
        let mut s = String::new();
        s.push(first);
        while let Some(&c) = self.peek() {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                s.push(self.next_char().unwrap());
            } else {
                break;
            }
        }
        s
    }

    fn consume_number(&mut self, first: char) -> f64 {
        let mut s = String::new();
        s.push(first);
        let mut has_dot = first == '.';
        while let Some(&c) = self.peek() {
            if c.is_ascii_digit() {
                s.push(self.next_char().unwrap());
            } else if c == '.' && !has_dot {
                has_dot = true;
                s.push(self.next_char().unwrap());
            } else {
                break;
            }
        }
        s.parse().unwrap_or(0.0)
    }

    fn consume_string(&mut self, quote: char) -> String {
        let mut s = String::new();
        loop {
            match self.next_char() {
                Some(c) if c == quote => return s,
                Some('\\') => {
                    if let Some(next) = self.next_char() {
                        s.push(next);
                    }
                }
                Some(c) => s.push(c),
                None => return s,
            }
        }
    }

    fn consume_hash(&mut self) -> String {
        let mut s = String::new();
        while let Some(&c) = self.peek() {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                s.push(self.next_char().unwrap());
            } else {
                break;
            }
        }
        s
    }

    pub fn next_token(&mut self) -> Token {
        self.consume_whitespace();

        let Some(c) = self.next_char() else {
            return Token::Eof;
        };

        match c {
            '/' if self.peek() == Some(&'*') => {
                self.next_char();
                self.consume_comment();
                Token::Comment
            }
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            ':' => Token::Colon,
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '.' => Token::Dot,
            '#' => {
                let name = self.consume_hash();
                if name.is_empty() {
                    Token::Delim('#')
                } else {
                    Token::Hash(name)
                }
            }
            '@' => {
                let name = self.consume_ident(' ');
                Token::AtKeyword(name.trim().to_string())
            }
            '"' | '\'' => Token::String(self.consume_string(c)),
            c if c.is_ascii_digit() || c == '.' || c == '+' || c == '-' => {
                let num = self.consume_number(c);
                // Check for dimension (number followed by ident)
                if let Some(&peek) = self.peek() {
                    if peek.is_alphabetic() || peek == '%' {
                        let next = self.next_char().unwrap();
                        let unit = self.consume_ident(next);
                        if unit == "%" {
                            Token::Percentage(num)
                        } else {
                            Token::Dimension(num, unit)
                        }
                    } else {
                        Token::Number(num)
                    }
                } else {
                    Token::Number(num)
                }
            }
            c if c.is_alphabetic() || c == '_' => {
                let ident = self.consume_ident(c);
                Token::Ident(ident)
            }
            c => Token::Delim(c),
        }
    }
}

/// Collecte tous les tokens d'une entrée CSS.
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tok = Tokenizer::new(input);
    let mut tokens = Vec::new();
    loop {
        let t = tok.next_token();
        if t == Token::Eof {
            break;
        }
        if !matches!(t, Token::Comment) {
            tokens.push(t);
        }
    }
    tokens
}
