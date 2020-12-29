#[derive(Debug, Clone, PartialEq)]
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
    Comment(String),
    StringLiteral(String),
    Integer(i32),
    // TODO: add support for negative numbers
    Name(String),
}

const KEYWORDS_MAP: [(&str, Token); 9] = [
    ("if", Token::If),
    ("else", Token::Else),
    ("return", Token::Return),
    ("fn", Token::Function),
    ("loop", Token::Loop),
    ("for", Token::For),
    ("in", Token::In),
    ("let", Token::Let),
    ("const", Token::Const),
];

pub struct Lexer {
    index: usize,
}

fn skip_first_and_get_all_chars_till(source: Vec<char>, terminator: char) -> Vec<char> {
    let mut index = 1;
    loop {
        if index < source.len() && source[index] != terminator {
            index += 1;
        } else {
            break;
        }
    }
    source[1..index].to_vec()
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer { index: 0 }
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
                    '\'' => Token::Quote, // StringLiteral(String), -> this should be handled when ' is detected
                    '"' => Token::DoubleQuote, // StringLiteral(String), -> this should be handled when " is detected
                    '!' => Token::ExclamationMark,
                    '#' => {
                        if code_chars.len() > self.index + 1 {
                            let comment_vec = skip_first_and_get_all_chars_till(
                                code_chars[self.index..].to_vec(),
                                '\n',
                            );
                            self.index += comment_vec.len() + 1; // We're adding skipped # character here
                            Token::Comment(comment_vec.into_iter().collect())
                        } else {
                            Token::Comment("".to_string())
                        }
                    }
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
                let keword_result = KEYWORDS_MAP.iter().position(|token| {
                    Lexer::contains(source_code.to_string(), token.0.to_string(), self.index)
                });
                let mut found_keyword = false; // TODO: fix this crap. Do a proper if/else if/else outside
                match keword_result {
                    Some(index) => {
                        tokens.push(KEYWORDS_MAP[index].1.clone());
                        self.index += KEYWORDS_MAP[index].0.len();
                        found_keyword = true;
                    }
                    None => (),
                };

                if !found_keyword {
                    // Integer(String),
                    if code_chars[self.index].is_numeric() {
                        let mut number_buffer = String::new();
                        loop {
                            if self.index < code_chars.len() && code_chars[self.index].is_numeric()
                            {
                                number_buffer.push(code_chars[self.index]);
                                self.index += 1;
                            } else {
                                break;
                            }
                        }
                        let parsed_number = number_buffer.parse().unwrap();
                        tokens.push(Token::Integer(parsed_number));
                    }
                }
            }
        }
    }

    fn contains(source: String, token: String, index: usize) -> bool {
        index + token.len() <= source.len() && source[index..index + token.len()] == token
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
    fn lexer_integers() {
        assert_eq!(lex("0".to_string()), Ok(vec![Token::Integer(0)]));
        assert_eq!(lex("1".to_string()), Ok(vec![Token::Integer(1)]));
        assert_eq!(lex("10".to_string()), Ok(vec![Token::Integer(10)]));
        assert_eq!(
            lex("2147483647".to_string()),
            Ok(vec![Token::Integer(std::i32::MAX)])
        );
        // TODO: is this approach good?
        assert_eq!(
            lex("-0".to_string()),
            Ok(vec![Token::Minus, Token::Integer(0)])
        );
        assert_eq!(
            lex("-1".to_string()),
            Ok(vec![Token::Minus, Token::Integer(1)])
        );
        assert_eq!(
            lex("-2".to_string()),
            Ok(vec![Token::Minus, Token::Integer(2)])
        );
        assert_eq!(
            lex("-45".to_string()),
            Ok(vec![Token::Minus, Token::Integer(45)])
        );
        assert_eq!(
            lex("-2147483647".to_string()),
            Ok(vec![Token::Minus, Token::Integer(std::i32::MAX)])
        );
    }

    #[test]
    fn lexer_comments() {
        assert_eq!(
            lex("# Example comment\n".to_string()),
            Ok(vec![Token::Comment(" Example comment".to_string())])
        );
    }

    // #[test]
    // fn lexer_multiple_tokens() {
    //     assert_eq!(lex("let x = 10;".to_string()), Ok(vec![Token::Let]));
    //     assert_eq!(lex("else".to_string()), Ok(vec![Token::Else]));
    //     // assert_eq!(lex("return".to_string()), Ok(vec![Token::Return]));
    //     assert_eq!(lex("fn".to_string()), Ok(vec![Token::Function]));
    //     assert_eq!(lex("loop".to_string()), Ok(vec![Token::Loop]));
    //     assert_eq!(lex("for".to_string()), Ok(vec![Token::For]));
    //     assert_eq!(lex("in".to_string()), Ok(vec![Token::In]));
    //     assert_eq!(lex("let".to_string()), Ok(vec![Token::Let]));
    //     assert_eq!(lex("const".to_string()), Ok(vec![Token::Const]));
    // }

    #[test]
    fn test_lexer_contains() {
        // Positive tests
        assert_eq!(
            Lexer::contains("let".to_string(), "let".to_string(), 0),
            true
        );
        assert_eq!(
            Lexer::contains(" let".to_string(), "let".to_string(), 1),
            true
        );
        assert_eq!(
            Lexer::contains(" let x".to_string(), "let".to_string(), 1),
            true
        );

        // Negative tests
        assert_eq!(Lexer::contains("".to_string(), "let".to_string(), 0), false);
        assert_eq!(
            Lexer::contains("".to_string(), "let".to_string(), 10),
            false
        );
        assert_eq!(
            Lexer::contains("l".to_string(), "let".to_string(), 0),
            false
        );
        assert_eq!(
            Lexer::contains("l".to_string(), "let".to_string(), 2),
            false
        );
        assert_eq!(
            Lexer::contains("l".to_string(), "let".to_string(), 3),
            false
        );
        assert_eq!(
            Lexer::contains("le".to_string(), "let".to_string(), 0),
            false
        );
    }
}
