use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use std::{collections::HashMap, iter::Peekable};

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

    Arrow,

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

lazy_static! {
    static ref TOKEN_MAPPINGS: HashMap<char, Token> = {
        let mut map = HashMap::new();
        map.insert('+', Token::Plus);
        map.insert('-', Token::Minus);
        map.insert('*', Token::Star);
        map.insert('/', Token::Slash);
        map.insert('.', Token::Dot);
        map.insert(',', Token::Comma);
        map.insert(':', Token::Colon);
        map.insert(';', Token::Semicolon);
        map.insert('=', Token::Assign);
        map.insert('>', Token::Greater);
        map.insert('<', Token::Lower);
        map.insert('!', Token::ExclamationMark);
        map.insert('{', Token::LeftBrace);
        map.insert('}', Token::RightBrace);
        map.insert('(', Token::LeftParenthesis);
        map.insert(')', Token::RightParenthesis);
        map.insert('[', Token::LeftSquareBracket);
        map.insert(']', Token::RightSquareBracket);
        map
    };
}

pub struct LexerIterator<I>
where
    I: Iterator<Item = char>,
{
    code_iter: Peekable<I>,
}

impl<I> LexerIterator<I>
where
    I: Iterator<Item = char>,
{
    fn parse_string_literal(&mut self, start_quote: char) -> Result<Token> {
        let mut string_literal = String::new();
        let mut escape = false;
        self.code_iter.next();

        while let Some(&c) = self.code_iter.peek() {
            self.code_iter.next();
            if escape {
                match c {
                    '"' => string_literal.push('"'),
                    '\'' => string_literal.push('\''),
                    '\\' => string_literal.push('\\'),
                    'n' => string_literal.push('\n'),
                    't' => string_literal.push('\t'),
                    _ => string_literal.push(c),
                }
                escape = false;
            } else if c == '\\' {
                escape = true;
            } else if c == start_quote {
                return Ok(Token::StringLiteral(string_literal));
            } else {
                string_literal.push(c);
            }
        }

        Err(anyhow::Error::msg("Unclosed string literal"))
    }

    fn parse_comment(&mut self) -> Result<Token> {
        let mut comment = String::new();
        self.code_iter.next();
        while let Some(&c) = self.code_iter.peek() {
            if c == '\n' || c == '\r' {
                break;
            } else {
                comment.push(c);
                self.code_iter.next();
            }
        }
        Ok(Token::Comment(comment))
    }

    fn parse_integer(&mut self, negative: bool) -> Result<Token> {
        let mut number: i64 = 0;
        while let Some(&c) = self.code_iter.peek() {
            if let Some(digit) = c.to_digit(10) {
                number = number * 10 + digit as i64;
                self.code_iter.next();
            } else {
                break;
            }
        }

        if negative {
            number = -number;
        }

        if number < i32::MIN as i64 || number > i32::MAX as i64 {
            return Err(anyhow::Error::msg("Integer overflow"));
        }

        Ok(Token::Integer(number as i32))
    }
    
    fn parse_minus(&mut self) -> Result<Token> {
        self.code_iter.next();
        if self.code_iter.peek().map_or(false, |c| c.is_digit(10)) {
            self.parse_integer(true)
        } else if let Some('>') = self.code_iter.peek() {
            self.code_iter.next();
            return Ok(Token::Arrow);
        } else {
            Ok(Token::Minus)
        }
    }

    fn parse_name(&mut self) -> Result<Token> {
        let mut identifier = String::new();
        while let Some(&c) = self.code_iter.peek() {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.code_iter.next();
            } else {
                break;
            }
        }

        let token = match identifier.as_str() {
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            "fn" => Token::Function,
            "loop" => Token::Loop,
            "for" => Token::For,
            "in" => Token::In,
            "let" => Token::Let,
            "const" => Token::Const,
            _ => Token::Name(identifier),
        };

        Ok(token)
    }
}

impl<I> Iterator for LexerIterator<I>
where
    I: Iterator<Item = char>,
{
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(&ch) = self.code_iter.peek() {
            if ch.is_whitespace() {
                self.code_iter.next();
                continue;
            }

            if ch.is_digit(10) {
                return Some(self.parse_integer(false));
            }

            if ch == '-' {
                return Some(self.parse_minus());
            }

            if ch.is_alphabetic() || ch == '_' {
                return Some(self.parse_name());
            }

            if ch == '"' || ch == '\'' {
                return Some(self.parse_string_literal(ch));
            }

            if ch == '#' {
                return Some(self.parse_comment());
            }

            if let Some(token) = TOKEN_MAPPINGS.get(&ch) {
                self.code_iter.next();
                return Some(Ok(token.clone()));
            }

            self.code_iter.next();
            return Some(Err(anyhow!("Unrecognized character")));
        }
        None
    }
}

#[derive(Default)]
pub struct Lexer {}

impl Lexer {
    pub fn tokenize<I>(source_code_iter: I) -> LexerIterator<I>
    where
        I: Iterator<Item = char>,
    {
        LexerIterator {
            code_iter: source_code_iter.peekable(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn lex(code: &str) -> Vec<Token> {
        Lexer::tokenize(code.chars()).map(|r| r.unwrap()).collect()
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
    #[test_case("->" => vec![Token::Arrow]; "arrow token")]
    fn symbol_token_tests(input: &str) -> Vec<Token> {
        lex(input)
    }

    #[test_case("if" => vec![Token::If]; "if token")]
    #[test_case("else" => vec![Token::Else]; "else token")]
    #[test_case("return" => vec![Token::Return]; "return token")]
    #[test_case("fn" => vec![Token::Function]; "fn token")]
    #[test_case("loop" => vec![Token::Loop]; "loop token")]
    #[test_case("for" => vec![Token::For]; "for token")]
    #[test_case("in" => vec![Token::In]; "in token")]
    #[test_case("let" => vec![Token::Let]; "let token")]
    #[test_case("const" => vec![Token::Const]; "const token")]
    fn lexer_keywords(input: &str) -> Vec<Token> {
        lex(input)
    }

    #[test_case("0" => vec![Token::Integer(0)]; "Integer 0")]
    #[test_case("1" => vec![Token::Integer(1)]; "Integer 1")]
    #[test_case("10" => vec![Token::Integer(10)]; "Integer 10")]
    #[test_case("2147483647" => vec![Token::Integer(std::i32::MAX)]; "Integer 2147483647")]
    fn lexer_positive_integers(input: &str) -> Vec<Token> {
        lex(input)
    }

    #[test_case("-0" => vec![Token::Integer(0)]; "Integer 0")]
    #[test_case("-1" => vec![Token::Integer(-1)]; "Integer -1")]
    #[test_case("-10" => vec![Token::Integer(-10)]; "Integer -10")]
    #[test_case("-2147483648" => vec![Token::Integer(std::i32::MIN)]; "Integer -2147483648")]
    fn lexer_negative_integers(input: &str) -> Vec<Token> {
        lex(input)
    }

    #[test_case("a" => vec![Token::Name("a".to_string())]; "a name")]
    #[test_case("v1" => vec![Token::Name("v1".to_string())]; "v1 name")]
    #[test_case("LongName_withDifferentStyles__" => vec![Token::Name("LongName_withDifferentStyles__".to_string())]; "LongName_withDifferentStyles__ name")]
    fn lexer_names(input: &str) -> Vec<Token> {
        lex(input)
    }

    #[test_case("# Example comment\n" => vec![Token::Comment(" Example comment".to_string())]; " Example comment")]
    fn lexer_comments(input: &str) -> Vec<Token> {
        lex(input)
    }

    #[test_case("\'Test string\'" => vec![Token::StringLiteral("Test string".to_string())]; "Test string in single quotas")]
    #[test_case("\"Test string\"" => vec![Token::StringLiteral("Test string".to_string())]; "Test string in double quotas")]
    #[test_case("\'Test \\\' string\'" => vec![Token::StringLiteral("Test \' string".to_string())]; "Test string in single quotas with escaped \'")]
    #[test_case("\"Test \\\" string\"" => vec![Token::StringLiteral("Test \" string".to_string())]; "Test string in double quotas with escaped \"")]
    fn lexer_string_literal(input: &str) -> Vec<Token> {
        lex(input)
    }

    #[test_case("let x = 10;" => vec![
        Token::Let, 
        Token::Name("x".to_string()), 
        Token::Assign, 
        Token::Integer(10), 
        Token::Semicolon
        ]; "Variable declaration")]
    #[test_case("# Example comment\n fn foo(a: i32, b: string) -> bool\n { return b + a; } " => vec![
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
        Token::Arrow,
        Token::Name("bool".to_string()),
        Token::LeftBrace,
        Token::Return,
        Token::Name("b".to_string()),
        Token::Plus,
        Token::Name("a".to_string()),
        Token::Semicolon,
        Token::RightBrace
    ]; "Function declaration with comments, parameters, return type, and body")]
    fn lexer_multi_token_cases(input: &str) -> Vec<Token> {
        lex(input)
    }
}
