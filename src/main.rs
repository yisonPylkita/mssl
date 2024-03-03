use crate::runtime::Runtime;

mod lexer;
mod runtime;

fn main() {
    let source_code = &r#"let x = 10; println("Hello there")"#.to_string();
    println!("Lexing code: {source_code}");

    let mut lex = lexer::Lexer::new();
    let ast = lex
        .tokenize(source_code)
        .expect("Could not parse this code");
    println!("{:?}", ast);
    println!("");

    println!("Running this code now");
    println!("----------------------");

    let mut runtime = Runtime::new();
    runtime.run(ast).expect("Cannot run this code");

    println!("----------------------");
    println!("");

    println!("Variables in runtime memory after script execution");
    for (k, v) in runtime.variables {
        println!("{k}: {v:?}");
    }
}
