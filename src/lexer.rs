#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Invalid,

    // symbols
    Plus,
    Minus,
    Star,
    Slash,
    BackSlash,
    Dot,
    Comma,
    Colon,
    Semicolon,
    Assign,
    Greater,
    Lower,
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

fn skip_first_and_get_all_chars_till(source: Vec<char>, terminators: Vec<char>) -> Vec<char> {
    let mut index = 1;
    loop {
        if index < source.len() && !terminators.contains(&source[index]) {
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
                    '*' => Token::Star,
                    '/' => Token::Slash,
                    '\\' => Token::BackSlash,
                    '.' => Token::Dot,
                    ',' => Token::Comma,
                    ':' => Token::Colon,
                    ';' => Token::Semicolon,
                    '=' => Token::Assign,
                    '>' => Token::Greater,
                    '<' => Token::Lower,
                     '\'' | '"' => {
                        if code_chars.len() > self.index + 1 {
                            let string_vec = skip_first_and_get_all_chars_till(
                                code_chars[self.index..].to_vec(),
                                vec!['\'', '"'],
                            );
                            self.index += string_vec.len() + 1; // We're adding skipped " character here
                            Token::StringLiteral(string_vec.into_iter().collect())
                        } else {
                            Token::StringLiteral("".to_string())
                        }
                    }
                    '!' => Token::ExclamationMark,
                    '#' => {
                        if code_chars.len() > self.index + 1 {
                            let comment_vec = skip_first_and_get_all_chars_till(
                                code_chars[self.index..].to_vec(),
                                vec!['\n'],
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
                    if code_chars[self.index].is_numeric() {
                        // Integer(String),
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
                    } else {
                        // Name(String)
                        let mut name_buffer = String::new();
                        loop {
                            if self.index < code_chars.len()
                                && (code_chars[self.index].is_ascii_alphanumeric()
                                    || code_chars[self.index] == '_')
                            {
                                name_buffer.push(code_chars[self.index]);
                                self.index += 1;
                            } else {
                                break;
                            }
                        }
                        tokens.push(Token::Name(name_buffer));
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
    use test_case::test_case;

    fn lex(code: String) -> Result<Vec<Token>, String> {
        let mut lex = Lexer::new();
        lex.tokenize(&code)
    }

    #[test_case("" => Vec::<Token>::new(); "empty string")]
    #[test_case(" " => Vec::<Token>::new(); "space is not a token")]
    #[test_case("let" => vec![Token::Let]; "let token")]
    #[test_case("+" => vec![Token::Plus]; "plus token")]
    #[test_case("-" => vec![Token::Minus]; "minus token")]
    #[test_case("*" => vec![Token::Star]; "star token")]
    #[test_case("/" => vec![Token::Slash]; "slash token")]
    #[test_case("\\" => vec![Token::BackSlash]; "backslash token")]
    #[test_case("," => vec![Token::Comma]; "comma token")]
    #[test_case(":" => vec![Token::Colon]; "colon token")]
    #[test_case(";" => vec![Token::Semicolon]; "semicolon token")]
    #[test_case("=" => vec![Token::Assign]; "assign token")]
    #[test_case(">" => vec![Token::Greater]; "greater token")]
    #[test_case("<" => vec![Token::Lower]; "lower token")]
    #[test_case("'" => vec![Token::StringLiteral("".to_string())]; "Single quote should be interpreted as beggining of StringLiteral")]
    #[test_case("\"" => vec![Token::StringLiteral("".to_string())]; "Single double quote should be interpreted as beggining of StringLiteral")]
    #[test_case("!" => vec![Token::ExclamationMark]; "exclamation mark token")]
    #[test_case("{" => vec![Token::LeftBrace]; "left brace token")]
    #[test_case("}" => vec![Token::RightBrace]; "right brace token")]
    #[test_case("(" => vec![Token::LeftParenthesis]; "left parenthesis token")]
    #[test_case(")" => vec![Token::RightParenthesis]; "right parenthesis token")]
    #[test_case("[" => vec![Token::LeftSquareBracket]; "left square bracket token")]
    #[test_case("]" => vec![Token::RightSquareBracket]; "right square bracket token")]

    fn multiplication_tests(input: &str) -> Vec<Token> {
        Lexer::new().tokenize(&input.to_string()).unwrap()
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
    fn lexer_keywords() {
        assert_eq!(lex("if".to_string()), Ok(vec![Token::If]));
        assert_eq!(lex("else".to_string()), Ok(vec![Token::Else]));
        assert_eq!(lex("return".to_string()), Ok(vec![Token::Return]));
        assert_eq!(lex("fn".to_string()), Ok(vec![Token::Function]));
        assert_eq!(lex("loop".to_string()), Ok(vec![Token::Loop]));
        assert_eq!(lex("for".to_string()), Ok(vec![Token::For]));
        assert_eq!(lex("in".to_string()), Ok(vec![Token::In]));
        assert_eq!(lex("let".to_string()), Ok(vec![Token::Let]));
        assert_eq!(lex("const".to_string()), Ok(vec![Token::Const]));
    }

    #[test]
    fn lexer_names() {
        assert_eq!(
            lex("v1".to_string()),
            Ok(vec![Token::Name("v1".to_string())])
        );
    }

    #[test]
    fn lexer_comments() {
        assert_eq!(
            lex("# Example comment\n".to_string()),
            Ok(vec![Token::Comment(" Example comment".to_string())])
        );
    }

    #[test]
    fn lexer_multiple_tokens() {
        assert_eq!(
            lex("let x = 10;".to_string()),
            Ok(vec![
                Token::Let,
                Token::Name("x".to_string()),
                Token::Assign,
                Token::Integer(10),
                Token::Semicolon
            ])
        );
    }

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

    #[test]
    fn string_literal() {
        assert_eq!(
            lex("\'Test string\'".to_string()),
            Ok(vec![Token::StringLiteral("Test string".to_string())])
        );

        assert_eq!(
            lex("\"Test string\"".to_string()),
            Ok(vec![Token::StringLiteral("Test string".to_string())])
        );
    }
}
