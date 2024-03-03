mod lexer;

fn main() {
    let source_code = &r#"let x = 10; println("Hello there")"#.to_string();
    println!("Lexing code: {source_code}");

    let mut lex = lexer::Lexer::new();
    let tokens = lex
        .tokenize(source_code)
        .expect("Could not parse this code");
    println!("{:?}", tokens);
}
