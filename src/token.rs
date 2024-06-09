use std::collections::HashMap;
use std::{fmt, num::ParseFloatError};

use once_cell::sync::Lazy;

pub static KEYWORDS: Lazy<HashMap<String, TokenType>> = Lazy::new(|| {
    let mut m: HashMap<String, TokenType> = HashMap::new();

    m.insert("and".to_string(), TokenType::And);
    m.insert("or".to_string(), TokenType::Or);
    m.insert("class".to_string(), TokenType::Class);
    m.insert("extends".to_string(), TokenType::Extends);
    m.insert("super".to_string(), TokenType::Super);
    m.insert("if".to_string(), TokenType::If);
    m.insert("else".to_string(), TokenType::Else);
    m.insert("true".to_string(), TokenType::True);
    m.insert("false".to_string(), TokenType::False);
    m.insert("fun".to_string(), TokenType::Fun);
    m.insert("return".to_string(), TokenType::Return);
    m.insert("nil".to_string(), TokenType::Nil);
    m.insert("print".to_string(), TokenType::Print);
    m.insert("this".to_string(), TokenType::This);
    m.insert("var".to_string(), TokenType::Var);
    m.insert("const".to_string(), TokenType::Const);
    m.insert("for".to_string(), TokenType::For);
    m.insert("while".to_string(), TokenType::While);
    m.insert("loop".to_string(), TokenType::Loop);

    return m;
});


#[derive(Debug, Copy, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,  // (
    RightParen, // )
    LeftBrace,  // {
    RightBrace, // }
    Comma,      // ,
    Dot,        // .
    Minus,      // -
    Plus,       // +
    Semicolon,  // ;
    Slash,      // /
    Star,       // *

    // One or two character tokens.
    Bang,         // !
    BangEqual,    // !=
    Equal,        // =
    EqualEqual,   // ==
    Greater,      // >
    GreaterEqual, // >=
    Less,         // <
    LessEqual,    // <=

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Or,
    Class,
    Extends,
    Super,
    If,
    Else,
    True,
    False,
    Fun,
    Return,
    Nil,
    Print,
    This,
    Var,
    Const,
    For,
    While,
    Loop,

    // EOF
    EOF,
}

#[derive(Debug)]
struct TokenPos {
    line: usize,
    line_index: usize,
    index: usize,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Box<dyn Literal>,
    pos: TokenPos,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: &str,
        literal: Box<dyn Literal>,
        line: usize,
        index: usize,
        line_index: usize,
    ) -> Self {
        Token {
            token_type,
            literal,
            lexeme: lexeme.to_string(),
            pos: TokenPos {
                line,
                index,
                line_index,
            },
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

pub trait Literal: fmt::Display + std::fmt::Debug {}

#[derive(Debug)]
pub struct EOFLiteral {}

impl EOFLiteral {
    pub fn new() -> Box<Self> {
        Box::new(EOFLiteral {})
    }
}

impl fmt::Display for EOFLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl Literal for EOFLiteral {}

/// NullLiteral
#[derive(Debug)]
pub struct NullLiteral {}

impl NullLiteral {
    pub fn new() -> Box<Self> {
        Box::new(NullLiteral {})
    }
}

impl fmt::Display for NullLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl Literal for NullLiteral {}

#[derive(Debug)]
pub struct StringLiteral {
    content: String,
}

impl StringLiteral {
    pub fn new(content: &str) -> Box<Self> {
        Box::new(StringLiteral {
            content: content.to_string(),
        })
    }
}

impl fmt::Display for StringLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Content: {}", self.content)
    }
}

impl Literal for StringLiteral {}

#[derive(Debug)]
pub struct NumberLiteral {
    number: f64,
}

impl NumberLiteral {
    pub fn new(number: &str) -> Result<Box<Self>, ParseFloatError> {
        match number.parse() {
            Ok(num) => Ok(Box::new(NumberLiteral { number: num })),
            Err(e) => Err(e),
        }
    }
}

impl fmt::Display for NumberLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Number: {}", self.number)
    }
}

impl Literal for NumberLiteral {}
