pub mod lexer;

fn main() {
    println!("Hello, world!");

    let lex = lexer::Lexer::new();
    lex.tokenize("let x = 10;".to_string()).expect("Could not parse this code");
}
