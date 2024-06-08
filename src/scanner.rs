use super::utils;
use core::fmt;
use std::{result::Result, f64};

pub struct Scanner {
    pub tokens: Vec<Token>,
    source: String,
    start: usize,
    current: usize,
    line: usize
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        return Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1
        }
    }

    pub fn scan_tokens(&mut self) -> Result<(), String> {
        while !self.is_at_end() {
            self.start = self.current;
            let _ = self.scan_token()?;
        }
        
        self.tokens.append(&mut vec![Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line: self.line
        }]);

        Ok(())
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len().try_into().unwrap()
    }

    fn scan_token(&mut self) -> Result<(), String> {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let next_token = if self.next_is('=') { TokenType::BangEqual } else { TokenType::Bang };
                self.add_token(next_token)
            },
            '=' => {
                let next_token = if self.next_is('=') { TokenType::EqualEqual } else { TokenType::Equal };
                self.add_token(next_token)
            },
            '<' => {
                let next_token = if self.next_is('=') { TokenType::LessEqual } else { TokenType::Less };
                self.add_token(next_token)
            },
            '>' => {
                let next_token = if self.next_is('=') { TokenType::GreaterEqual } else { TokenType::Greater };
                self.add_token(next_token)
            },
            '/' => {
                let is_comment = self.next_is('/');
                if is_comment {
                    // NB, we don't need to add the token if it's a comment
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    };
                } else {
                    self.add_token(TokenType::Slash)
                }
            },
            ' ' | '\r' | '\t' => (),
            '\n' => self.line = self.line + 1,
            '"' => {
                let literal = self.string()?;
                self.add_token_literal(TokenType::String, literal);
            },
            c if utils::is_digit(c) => {
                let literal = self.number()?;
                self.add_token_literal(TokenType::Number, literal)
            },
            c if utils::is_alpha(c) => {
                let text = self.identifier();
                let token_type = match text.as_str() {
                    "and"    => TokenType::And,
                    "class"  => TokenType::Class,
                    "else"   => TokenType::Else,
                    "false"  => TokenType::False,
                    "for"    => TokenType::For,
                    "fun"    => TokenType::Fun,
                    "if"     => TokenType::If,
                    "nil"    => TokenType::Nil,
                    "or"     => TokenType::Or,
                    "print"  => TokenType::Print,
                    "return" => TokenType::Return,
                    "super"  => TokenType::Super,
                    "this"   => TokenType::This,
                    "true"   => TokenType::True,
                    "var"    => TokenType::Var,
                    "while"  => TokenType::While,
                    _        => TokenType::Identifier
                };

                self.add_token(token_type);
            },
            c => {
                utils::error(self.line, format!("Unexpected character '{}'", c))?;
            }
        }
        return Ok(())
    }

    fn advance(&mut self) -> char {
        if let Some(res) = self.source.chars().nth(self.current) {
            self.current = self.current + 1;
            return res
        } else {
            panic!("Advance requested but no character at position {}", self.current)
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.append(&mut vec![Token {
            token_type: token_type,
            lexeme: self.token_text(),
            literal: None,
            line: self.line
        }])
    }

    fn substring(&self, start: usize, end: usize) -> String {
        return self.source[start..end].to_string()
    }

    fn token_text(&self) -> String {
        return self.substring(self.start, self.current)
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: String) {
        let text = &self.source[self.start..self.current];
        self.tokens.append(&mut vec![Token {
            token_type: token_type,
            lexeme: text.to_string(),
            literal: Some(literal),
            line: self.line
        }])
    }

    fn next_is(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false
        };

        if self.source.chars().nth(self.current) != Some(expected) {
            return false
        };

        self.current = self.current + 1;

        return true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0'
        }
        if let Some(x) = self.source.chars().nth(self.current) {
            return x
        };
        panic!("Unreachable character")
    }

    fn peek_n(&self, n: usize) -> char {
        if (self.current + n) >= self.source.len() {
            return '\0' 
        };
        if let Some(x) = self.source.chars().nth(self.current + n) {
            return x
        };
        panic!("Unreachable character")
    }

    fn string(&mut self) -> Result<String, String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line = self.line + 1;
            };
            self.advance();
        };

        if self.is_at_end() {
            utils::error(self.line, String::from("Unterminated string"))?
        };

        self.advance();

        let out = self.substring(self.start + 1, self.current - 1);

        return Ok(out)
    }

    fn number(&mut self) -> Result<String, String> {
        while utils::is_digit(self.peek()) {
            self.advance();
        };

        if self.peek() == '.' && utils::is_digit(self.peek_n(1)) {
            // Consume the `.`
            self.advance();
            while utils::is_digit(self.peek()) {
                self.advance();
            };
        };

        let out = self.token_text();

        // It's definitely parsable. Real parsing happens later.
        if let Ok(_) = out.parse::<f64>() {
            return Ok(out)
        };

        panic!("Failed to parse number from string '{}'", out);
    }

    fn identifier(&mut self) -> String {
        while utils::is_alphanumeric(self.peek()) {
            self.advance();
        };
        return self.token_text()
    }
}

#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Eof
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Just call the debug method for now
        write!(f, "{}", format!("{:?}", self))
    }
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: usize
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.literal {
            Some(x) => write!(f, "{} {} {}", self.token_type, self.lexeme, x),
            None => write!(f, "{} {}", self.token_type, self.lexeme),
        }
        
    }
}




