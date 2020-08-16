#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
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
    Hash,
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

    pub fn tokenize(&mut self, source_code: &String) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();

        // How to proceed?
        // First check if char under index is a single char
        // Second check if there is a keyword starting from index - like 'let' or 'return' or 'fn'.
        // Then go to appropriate state

        let code_chars: Vec<char> = source_code.chars().collect();
        self.index = 0;
        loop {
            if self.index >= source_code.len() {
                return Ok(tokens);
            }
            // Sign case
            if code_chars[self.index] == ' ' {
                self.index += 1;
                continue;
            }
            // TODO: add UTF-8 string support
            if code_chars[self.index].is_ascii_punctuation() {
                let found_token = match code_chars[self.index] {
                    // Actually in some cases we should do more here. For example with " or '
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
                    '#' => Token::Hash,
                    '{' => Token::LeftBrace,
                    '}' => Token::RightBrace,
                    '(' => Token::LeftParenthesis,
                    ')' => Token::RightParenthesis,
                    '[' => Token::LeftSquareBracket,
                    ']' => Token::RightSquareBracket,
                    _ => Token::Invalid,
                };
                self.index += 1;
                tokens.push(found_token);
            } else {
                for checked_token in [
                    ("if", Token::If), ("else", Token::Else), ("return", Token::Return),
                    ("fn", Token::Function), ("loop", Token::Loop), ("for", Token::For),
                    ("in", Token::In), ("let", Token::Let), ("const", Token::Const)].iter() {
                    if Lexer::contains(source_code.to_string(), checked_token.0.to_string(), self.index) {
                        tokens.push(checked_token.1.clone());
                        self.index += checked_token.0.len();
                        break;
                    }
                }
            }
        }
    }

    fn contains(source: String, token: String, index: usize) -> bool {
        index + token.len() <= source.len() && source[index..index+token.len()] == token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(code: String) -> Result<Vec<Token>, String> {
        let mut lex = Lexer::new();
        lex.tokenize(&code)
    }
    #[test]
    fn lexer_single_sign() {
        assert_eq!(lex("".to_string()), Ok(vec![]));
        assert_eq!(lex(" ".to_string()), Ok(vec![]));
        assert_eq!(lex("let".to_string()), Ok(vec![Token::Let]));
        assert_eq!(lex("+".to_string()), Ok(vec![Token::Plus]));
        assert_eq!(lex("-".to_string()), Ok(vec![Token::Minus]));
        assert_eq!(lex("*".to_string()), Ok(vec![Token::Multiply]));
        assert_eq!(lex("/".to_string()), Ok(vec![Token::Slash]));
        assert_eq!(lex("\\".to_string()), Ok(vec![Token::BackSlash]));
        assert_eq!(lex(".".to_string()), Ok(vec![Token::Dot]));
        assert_eq!(lex(",".to_string()), Ok(vec![Token::Comma]));
        assert_eq!(lex(":".to_string()), Ok(vec![Token::Colon]));
        assert_eq!(lex(";".to_string()), Ok(vec![Token::Semicolon]));
        assert_eq!(lex("=".to_string()), Ok(vec![Token::Assign]));
        assert_eq!(lex(">".to_string()), Ok(vec![Token::Greater]));
        assert_eq!(lex("<".to_string()), Ok(vec![Token::Lower]));
        assert_eq!(lex("\"".to_string()), Ok(vec![Token::DoubleQuote]));
        assert_eq!(lex("\'".to_string()), Ok(vec![Token::Quote]));
        assert_eq!(lex("!".to_string()), Ok(vec![Token::ExclamationMark]));
        assert_eq!(lex("#".to_string()), Ok(vec![Token::Hash]));
        assert_eq!(lex("{".to_string()), Ok(vec![Token::LeftBrace]));
        assert_eq!(lex("}".to_string()), Ok(vec![Token::RightBrace]));
        assert_eq!(lex("(".to_string()), Ok(vec![Token::LeftParenthesis]));
        assert_eq!(lex(")".to_string()), Ok(vec![Token::RightParenthesis]));
        assert_eq!(lex("[".to_string()), Ok(vec![Token::LeftSquareBracket]));
        assert_eq!(lex("]".to_string()), Ok(vec![Token::RightSquareBracket]));
    }

    #[test]
    fn lexer_multicharacter_tokens() {
        assert_eq!(lex("if".to_string()), Ok(vec![Token::If]));
        assert_eq!(lex("else".to_string()), Ok(vec![Token::Else]));
        // assert_eq!(lex("return".to_string()), Ok(vec![Token::Return]));
        assert_eq!(lex("fn".to_string()), Ok(vec![Token::Function]));
        assert_eq!(lex("loop".to_string()), Ok(vec![Token::Loop]));
        assert_eq!(lex("for".to_string()), Ok(vec![Token::For]));
        assert_eq!(lex("in".to_string()), Ok(vec![Token::In]));
        assert_eq!(lex("let".to_string()), Ok(vec![Token::Let]));
        assert_eq!(lex("const".to_string()), Ok(vec![Token::Const]));
    }

    #[test]
    fn test_lexer_contains() {
        // Positive tests
        assert_eq!(Lexer::contains("let".to_string(), "let".to_string(), 0), true);
        assert_eq!(Lexer::contains(" let".to_string(), "let".to_string(), 1), true);
        assert_eq!(Lexer::contains(" let x".to_string(), "let".to_string(), 1), true);

        // Negative tests
        assert_eq!(Lexer::contains("".to_string(), "let".to_string(), 0), false);
        assert_eq!(Lexer::contains("".to_string(), "let".to_string(), 10), false);
        assert_eq!(Lexer::contains("l".to_string(), "let".to_string(), 0), false);
        assert_eq!(Lexer::contains("l".to_string(), "let".to_string(), 2), false);
        assert_eq!(Lexer::contains("l".to_string(), "let".to_string(), 3), false);
        assert_eq!(Lexer::contains("le".to_string(), "let".to_string(), 0), false);
    }
}