use crate::error::Error;
use crate::token::{
    EOFLiteral, Literal, NullLiteral, NumberLiteral, StringLiteral, Token, TokenType, KEYWORDS,
};

fn is_digit(ch: char) -> bool {
    ch >= '0' && ch <= '9'
}

fn is_alpha(ch: char) -> bool {
    (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || ch == '_'
}

fn is_alpha_digit(ch: char) -> bool {
    is_digit(ch) || is_alpha(ch)
}

pub struct Scanner {
    source: String,
    source_vec: Vec<char>,
    tokens: Vec<Token>,

    start: usize,
    curr: usize,
    line: usize,
    line_index: usize,

    errors: Vec<Error>,
}

impl Scanner {
    // Constructor
    pub fn new(source: &str) -> Self {
        return Scanner {
            source: source.to_string(),
            source_vec: source.chars().collect(),
            tokens: vec![],
            start: 0,
            curr: 0,
            line: 1,
            line_index: 0,
            errors: vec![],
        };
    }

    // Scan until end.
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.curr;
            self.scan();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "",
            EOFLiteral::new(),
            self.line,
            self.start,
            self.line_index,
        ));

        &self.tokens
    }

    // Scan one token.
    fn scan(&mut self) {
        let ch: char = self.advance();
        dbg!(ch);
        match ch {
            // Single char token.
            '(' => self.add_null_literal_token(TokenType::LeftBrace),
            ')' => self.add_null_literal_token(TokenType::RightBrace),
            '{' => self.add_null_literal_token(TokenType::LeftParen),
            '}' => self.add_null_literal_token(TokenType::RightParen),
            ',' => self.add_null_literal_token(TokenType::Comma),
            '.' => self.add_null_literal_token(TokenType::Dot),
            '-' => self.add_null_literal_token(TokenType::Minus),
            '+' => self.add_null_literal_token(TokenType::Plus),
            ';' => self.add_null_literal_token(TokenType::Semicolon),
            '*' => self.add_null_literal_token(TokenType::Star),

            // One or two character tokens.
            '!' => match self.match_next('=') {
                true => self.add_null_literal_token(TokenType::BangEqual),
                false => self.add_null_literal_token(TokenType::Bang),
            },
            '=' => match self.match_next('=') {
                true => self.add_null_literal_token(TokenType::EqualEqual),
                false => self.add_null_literal_token(TokenType::Equal),
            },
            '<' => match self.match_next('=') {
                true => self.add_null_literal_token(TokenType::LessEqual),
                false => self.add_null_literal_token(TokenType::Less),
            },
            '>' => match self.match_next('=') {
                true => self.add_null_literal_token(TokenType::GreaterEqual),
                false => self.add_null_literal_token(TokenType::Greater),
            },

            // Slash
            '/' => match self.match_next('/') {
                true => {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                false => {
                    self.add_null_literal_token(TokenType::Slash);
                }
            },

            // Whitespace char
            ' ' => (),
            // Other whitespace char
            '\t' => (),
            '\r' => (),
            '\n' => {
                self.line += 1;
                self.line_index = 0;
            }

            // String
            '"' => self.match_string(),

            // Numbers, Identifier, Keyword and error.
            others => {
                if is_digit(others) {
                    // Numebers
                    self.match_number();
                } else if is_alpha(ch) {
                    self.match_identifier();
                } else {
                    // Error
                    self.errors.push(Error::new(
                        self.line,
                        self.line_index,
                        "Unexpect character.",
                    ));
                }
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.curr >= self.source_vec.len()
    }

    fn advance(&mut self) -> char {
        self.curr += 1;
        self.line_index += 1;
        *self.source_vec.get(self.curr - 1).unwrap()
    }

    // peek one
    fn peek(&self) -> char {
        match self.is_at_end() {
            true => '\0',
            false => *self.source_vec.get(self.curr).unwrap(),
        }
    }

    // peek two
    fn peek_behind(&self) -> char {
        if self.curr + 1 >= self.source_vec.len() {
            return '\0';
        }
        return *self.source_vec.get(self.curr + 1).unwrap();
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if *self.source_vec.get(self.curr).unwrap() != expected {
            return false;
        }

        self.curr += 1;
        self.line_index += 1;
        return true;
    }

    fn match_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.errors.push(Error::new(
                self.line,
                self.line_index,
                "Unterminated string.",
            ));
            return;
        }

        self.advance();

        let value: String = self.source_vec[self.start + 1..self.curr - 1]
            .iter()
            .collect();
        self.add_token(TokenType::String, StringLiteral::new(&value));
    }

    fn match_number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_behind()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }

        let value: String = self.source_vec[self.start..self.curr].iter().collect();
        self.add_token(TokenType::Number, NumberLiteral::new(&value).unwrap())
    }

    fn match_identifier(&mut self) {
        while is_alpha_digit(self.peek()) {
            self.advance();
        }

        let text: String = self.source_vec[self.start..self.curr].iter().collect();
        match KEYWORDS.get(&text) {
            Some(keyword) => {
                self.add_null_literal_token(*keyword);
            }
            None => {
                self.add_null_literal_token(TokenType::Identifier);
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType, literal: Box<dyn Literal>) {
        let text: String = self.source_vec[self.start..self.curr].iter().collect();
        self.tokens.push(Token::new(
            token_type,
            &text,
            literal,
            self.line,
            self.start,
            self.line_index,
        ))
    }

    fn add_null_literal_token(&mut self, token_type: TokenType) {
        self.add_token(token_type, NullLiteral::new())
    }
}
