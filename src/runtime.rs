use crate::lexer::Token;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Value {
    Integer(i32),
    String(String),
}

#[derive(Default)]
pub struct Runtime {
    pub variables: HashMap<String, Value>,
}

impl Runtime {
    pub fn run(&mut self, ast: Vec<Token>) {
        let mut ast_iter = ast.iter().peekable();

        while let Some(token) = ast_iter.next() {
            match token {
                Token::Let => {
                    let var_name = match ast_iter.next().unwrap() {
                        Token::Name(name) => name.clone(),
                        _ => panic!("Variable name expected"),
                    };
                    match ast_iter.next().unwrap() {
                        Token::Assign => {}
                        _ => panic!("= sign expected"),
                    }
                    let value = match ast_iter.next().unwrap() {
                        Token::Integer(val) => Value::Integer(*val),
                        Token::StringLiteral(val) => Value::String(val.clone()),
                        _ => panic!("Only simple assignments of int or string are supported now"),
                    };
                    if let Some(Token::Semicolon) = ast_iter.peek() {
                        ast_iter.next().unwrap();
                    }

                    self.variables.insert(var_name, value);
                }
                Token::Name(name) => {
                    if name == "println" {
                        match ast_iter.next().unwrap() {
                            Token::LeftParenthesis => {}
                            _ => panic!("( sign expected"),
                        };
                        let msg_to_print = match ast_iter.next().unwrap() {
                            Token::Integer(msg) => msg.to_string(),
                            Token::StringLiteral(msg) => msg.clone(),
                            _ => panic!("String literal expected"),
                        };
                        match ast_iter.next().unwrap() {
                            Token::RightParenthesis => {}
                            _ => panic!(") sign expected"),
                        };
                        if let Some(Token::Semicolon) = ast_iter.peek() {
                            ast_iter.next().unwrap();
                        }

                        println!("{}", msg_to_print);
                    }
                }
                _ => {}
            }
        }
    }
}
