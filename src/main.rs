use clap::{Parser, Subcommand};

use crate::runtime::Runtime;

mod lexer;
mod runtime;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Lex source code
    Lex { source_code: String },

    /// Run source code
    Run { source_code: String },
}

fn main() {
    let args = Args::parse();
    match &args.command {
        Commands::Lex { source_code } => {
            let mut lex = lexer::Lexer::default();
            let ast = lex
                .tokenize(source_code)
                .expect("Could not parse this code");
            println!("{:?}", ast);
        }
        Commands::Run { source_code } => {
            let mut lex = lexer::Lexer::default();
            let ast = lex
                .tokenize(source_code)
                .expect("Could not parse this code");

            let mut runtime = Runtime::default();
            println!("----------------------");
            runtime.run(ast);
            println!("----------------------");
            println!("");

            println!("Variables in runtime memory after script execution");
            for (k, v) in runtime.variables {
                println!("{k}: {v:?}");
            }
        }
    }
}
