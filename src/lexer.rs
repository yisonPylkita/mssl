use itertools::Itertools;
use phf::phf_map;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // symbols
    Plus,
    Minus,
    Star,
    Slash,
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
    Name(String),
}

static TOKEN_MAPPINGS: phf::Map<char, Token> = phf_map! {
    '+' => Token::Plus,
    '-' => Token::Minus,
    '*' => Token::Star,
    '/' => Token::Slash,
    '.' => Token::Dot,
    ',' => Token::Comma,
    ':' => Token::Colon,
    ';' => Token::Semicolon,
    '=' => Token::Assign,
    '>' => Token::Greater,
    '<' => Token::Lower,
    '!' => Token::ExclamationMark,
    '{' => Token::LeftBrace,
    '}' => Token::RightBrace,
    '(' => Token::LeftParenthesis,
    ')' => Token::RightParenthesis,
    '[' => Token::LeftSquareBracket,
    ']' => Token::RightSquareBracket,
};

static KEYWORD_MAPPINGS: phf::Map<&'static str, Token> = phf_map! {
    "if" => Token::If,
    "else" => Token::Else,
    "return" => Token::Return,
    "fn" => Token::Function,
    "loop" => Token::Loop,
    "for" => Token::For,
    "in" => Token::In,
    "let" => Token::Let,
    "const" => Token::Const,
};

#[derive(Default)]
pub struct Lexer {}

impl Lexer {
    pub fn tokenize(&mut self, source_code: &str) -> Result<Vec<Token>, &str> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut code_iter = source_code.chars().peekable();

        while let Some(ch) = code_iter.peek() {
            if let Some(token) = TOKEN_MAPPINGS.get(ch).cloned() {
                tokens.push(token);
                code_iter
                    .next()
                    .expect("Logic error. This stream should have at least one character left");
            } else {
                match ch {
                    'a'..='z' | 'A'..='Z' => {
                        let name: String = code_iter
                            .peeking_take_while(|c| c.is_ascii_alphanumeric())
                            .collect();
                        if let Some(keyword) = KEYWORD_MAPPINGS.get(&name).cloned() {
                            tokens.push(keyword);
                        } else {
                            tokens.push(Token::Name(name));
                        }
                    }
                    '#' => {
                        code_iter
                            .next()
                            .expect("Logic error. There should be a # character in stream now");
                        let comment: String =
                            code_iter.peeking_take_while(|c| *c != '\n').collect();
                        tokens.push(Token::Comment(comment));
                    }
                    '"' => {
                        code_iter
                            .next()
                            .expect("Logic error. There should be a \" character in stream now");
                        let string_literal = code_iter.peeking_take_while(|c| *c != '"').collect();
                        code_iter.next().ok_or("End of file")?;
                        tokens.push(Token::StringLiteral(string_literal));
                    }
                    '\'' => {
                        code_iter
                            .next()
                            .expect("Logic error. There should be a ' character in stream now");
                        let string_literal = code_iter.peeking_take_while(|c| *c != '\'').collect();
                        code_iter.next().ok_or("End of file")?;
                        tokens.push(Token::StringLiteral(string_literal));
                    }
                    '0'..='9' => {
                        // TODO: add support for negative numbers
                        let number: Cow<str> =
                            code_iter.peeking_take_while(|c| c.is_numeric()).collect();
                        tokens.push(Token::Integer(
                            number.parse().map_err(|_err| "Couldn't parse number")?,
                        ));
                    }
                    _ => {
                        code_iter.next().expect(
                            "Logic error. This stream should have at least one character left",
                        );
                    }
                }
            }
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn lex(code: &str) -> Result<Vec<Token>, String> {
        let mut lex = Lexer::default();
        lex.tokenize(code).map_err(|err| err.to_string())
    }

    #[test_case("" => Vec::<Token>::new(); "empty string")]
    #[test_case(" " => Vec::<Token>::new(); "space is not a token")]
    #[test_case("let" => vec![Token::Let]; "let token")]
    #[test_case("+" => vec![Token::Plus]; "plus token")]
    #[test_case("-" => vec![Token::Minus]; "minus token")]
    #[test_case("*" => vec![Token::Star]; "star token")]
    #[test_case("/" => vec![Token::Slash]; "slash token")]
    #[test_case("," => vec![Token::Comma]; "comma token")]
    #[test_case(":" => vec![Token::Colon]; "colon token")]
    #[test_case(";" => vec![Token::Semicolon]; "semicolon token")]
    #[test_case("=" => vec![Token::Assign]; "assign token")]
    #[test_case(">" => vec![Token::Greater]; "greater token")]
    #[test_case("<" => vec![Token::Lower]; "lower token")]
    #[test_case("!" => vec![Token::ExclamationMark]; "exclamation mark token")]
    #[test_case("{" => vec![Token::LeftBrace]; "left brace token")]
    #[test_case("}" => vec![Token::RightBrace]; "right brace token")]
    #[test_case("(" => vec![Token::LeftParenthesis]; "left parenthesis token")]
    #[test_case(")" => vec![Token::RightParenthesis]; "right parenthesis token")]
    #[test_case("[" => vec![Token::LeftSquareBracket]; "left square bracket token")]
    #[test_case("]" => vec![Token::RightSquareBracket]; "right square bracket token")]

    fn multiplication_tests(input: &str) -> Vec<Token> {
        Lexer::default().tokenize(&input).unwrap()
    }

    #[test]
    fn lexer_multicharacter_tokens() {
        assert_eq!(lex("if"), Ok(vec![Token::If]));
        assert_eq!(lex("else"), Ok(vec![Token::Else]));
        assert_eq!(lex("return"), Ok(vec![Token::Return]));
        assert_eq!(lex("fn"), Ok(vec![Token::Function]));
        assert_eq!(lex("loop"), Ok(vec![Token::Loop]));
        assert_eq!(lex("for"), Ok(vec![Token::For]));
        assert_eq!(lex("in"), Ok(vec![Token::In]));
        assert_eq!(lex("let"), Ok(vec![Token::Let]));
        assert_eq!(lex("const"), Ok(vec![Token::Const]));
    }

    #[test]
    fn lexer_integers() {
        assert_eq!(lex("0"), Ok(vec![Token::Integer(0)]));
        assert_eq!(lex("1"), Ok(vec![Token::Integer(1)]));
        assert_eq!(lex("10"), Ok(vec![Token::Integer(10)]));
        assert_eq!(lex("2147483647"), Ok(vec![Token::Integer(std::i32::MAX)]));
        // TODO: add support for negative numbers
        assert_eq!(lex("-0"), Ok(vec![Token::Minus, Token::Integer(0)]));
        assert_eq!(lex("-1"), Ok(vec![Token::Minus, Token::Integer(1)]));
        assert_eq!(lex("-2"), Ok(vec![Token::Minus, Token::Integer(2)]));
        assert_eq!(lex("-45"), Ok(vec![Token::Minus, Token::Integer(45)]));
        assert_eq!(
            lex("-2147483647"),
            Ok(vec![Token::Minus, Token::Integer(std::i32::MAX)])
        );
    }

    #[test]
    fn lexer_keywords() {
        assert_eq!(lex("if"), Ok(vec![Token::If]));
        assert_eq!(lex("else"), Ok(vec![Token::Else]));
        assert_eq!(lex("return"), Ok(vec![Token::Return]));
        assert_eq!(lex("fn"), Ok(vec![Token::Function]));
        assert_eq!(lex("loop"), Ok(vec![Token::Loop]));
        assert_eq!(lex("for"), Ok(vec![Token::For]));
        assert_eq!(lex("in"), Ok(vec![Token::In]));
        assert_eq!(lex("let"), Ok(vec![Token::Let]));
        assert_eq!(lex("const"), Ok(vec![Token::Const]));
    }

    #[test]
    fn lexer_names() {
        assert_eq!(lex("v1"), Ok(vec![Token::Name("v1".to_string())]));
    }

    #[test]
    fn lexer_comments() {
        assert_eq!(
            lex("# Example comment\n"),
            Ok(vec![Token::Comment(" Example comment".to_string())])
        );
    }

    #[test]
    fn lexer_multiple_tokens() {
        assert_eq!(
            lex("let x = 10;"),
            Ok(vec![
                Token::Let,
                Token::Name("x".to_string()),
                Token::Assign,
                Token::Integer(10),
                Token::Semicolon
            ])
        );

        assert_eq!(
            lex("# Example comment\n fn foo(a: i32, b: string) -> bool\n { return b + a; } "),
            Ok(vec![
                Token::Comment(" Example comment".to_string()),
                Token::Function,
                Token::Name("foo".to_string()),
                Token::LeftParenthesis,
                Token::Name("a".to_string()),
                Token::Colon,
                Token::Name("i32".to_string()),
                Token::Comma,
                Token::Name("b".to_string()),
                Token::Colon,
                Token::Name("string".to_string()),
                Token::RightParenthesis,
                Token::Minus,
                Token::Greater,
                Token::Name("bool".to_string()),
                Token::LeftBrace,
                Token::Return,
                Token::Name("b".to_string()),
                Token::Plus,
                Token::Name("a".to_string()),
                Token::Semicolon,
                Token::RightBrace
            ])
        );
    }

    #[test]
    fn string_literal() {
        assert_eq!(
            lex("\'Test string\'"),
            Ok(vec![Token::StringLiteral("Test string".to_string())])
        );

        assert_eq!(
            lex("\"Test string\""),
            Ok(vec![Token::StringLiteral("Test string".to_string())])
        );
    }
}
