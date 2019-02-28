use std::str::Chars;
use std::iter::Peekable;

pub enum Token {
    Invalid,

    // symbols
    Plus,
    Minus,
    Multiply,
    Slash,
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    Dot,
    Greater,
    Lower,

    // keywords
    If,
    Else,
    Return,
    Function,
    Loop,
    For,
    In,
    Let,
    Const,

    // Others
    StringLiteral(String),
    Integer(String),
    Float(String),
    Name(String),
}

pub struct Lexer {
    index: usize,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            index: 0,
        }
    }

    pub fn tokenize(&self, source_code: String) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();
        loop {
            if self.index > source_code.len() {
                return Ok(tokens);
            }

            let current_char = source_code.chars().nth(self.index).unwrap();
            let found_token = match current_char {
                '+' => Token::Plus,
                _ => Token::Invalid,
            };

            tokens.push(found_token);
        }
    }
}
