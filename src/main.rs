pub mod lexer;

fn main() {
    let mut lex = lexer::Lexer::new();
    let tokens = lex.tokenize(r#"let x = 10; println("Hello there")"#.to_string())
        .expect("Could not parse this code");
    print!("{:?}", tokens);
}
