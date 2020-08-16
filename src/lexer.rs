use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug)]
pub enum Token {
    Invalid,

    // symbols
    Plus,
    Minus,
    Multiply,
    Slash,
    BackSlash,
    Dot,
    Comma,
    Colon,
    Semicolon,
    Assign,
    Greater,
    Lower,
    Quote,
    DoubleQuote,
    ExclamationMark,
    LeftBrace,
    RightBrace,
    LeftParenthesis,
    RightParenthesis,
    LeftSquareBracket,
    RightSquareBracket,

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

    pub fn tokenize(&mut self, source_code: String) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();

        let code_chars: Vec<char> = source_code.chars().collect();
        for current_char in code_chars {
            let found_token = match current_char {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Multiply,
                '/' => Token::Slash,
                '\\' => Token::BackSlash,
                '.' => Token::Dot,
                ',' => Token::Comma,
                ':' => Token::Colon,
                ';' => Token::Semicolon,
                '=' => Token::Assign,
                '>' => Token::Greater,
                '<' => Token::Lower,
                '\'' => Token::Quote,
                '"' => Token::DoubleQuote,
                '!' => Token::ExclamationMark,
                '{' => Token::LeftBrace,
                '}' => Token::RightBrace,
                '(' => Token::LeftParenthesis,
                ')' => Token::RightParenthesis,
                '[' => Token::LeftSquareBracket,
                ']' => Token::RightSquareBracket,
                _ => Token::Invalid,
            };

            tokens.push(found_token);
        }
        return Ok(tokens);
    }
}
