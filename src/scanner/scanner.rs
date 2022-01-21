use crate::error;
use crate::lexer::token;
use crate::lexer::token::{Token, TokenType};

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::EOF, "", self.line));
        &self.tokens
    }

    pub fn scan_token(&mut self) {
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
                if self.matches('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                };
            }
            '=' => {
                if self.matches('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.matches('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.matches('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        print!("Comment");
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            '"' => self.add_string(),
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            _ => {
                if c.is_digit(10) {
                    self.add_number();
                } else if c.is_alphabetic() {
                    self.add_identifier();
                } else {
                    error(self.line, "Unexpected character");
                }
            }
        }
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }

    fn add_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Unterminated string");
            return;
        }

        self.advance();

        let literal = self
            .source
            .get(self.start + 1..self.current - 1)
            .expect("[String] Could not get substring")
            .to_string();

        self.add_token(TokenType::String { literal });
    }

    fn add_number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let literal: f64 = self
            .source
            .get(self.start..self.current)
            .expect("[Number] Could not get substring")
            .parse()
            .expect("[Number] Could not parse number");

        self.add_token(TokenType::Number { literal });
    }

    fn add_identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }
        let literal = self
            .source
            .get(self.start..self.current)
            .expect("Could not get identifier");

        let identifier_type = token::match_keywords(literal).unwrap_or(TokenType::Identifier);

        self.add_token(identifier_type);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let current_char = self
            .source
            .chars()
            .nth(self.current)
            .expect("Unexpected end of source");

        self.current += 1;
        current_char
    }

    fn add_token(&mut self, type_: TokenType) {
        let text = self.source.get(self.start..self.current).unwrap_or("");
        self.tokens.push(Token::new(type_, text, self.line));
    }
}
