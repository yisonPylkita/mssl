use crate::runtime::Runtime;
use anyhow::Result;
use clap::{Parser, Subcommand};
use lexer::{Lexer, Token};
use std::{fs, path::PathBuf};

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
    Lex {
        /// Source code file path
        #[clap(long, short = 'f')]
        file: PathBuf,

        /// Verbose
        #[clap(long, short = 'v', action = clap::ArgAction::Count)]
        verbose: u8,
    },

    /// Run source code
    Run {
        /// Source code file path
        #[clap(long, short = 'f')]
        file: PathBuf,

        /// Verbose
        #[clap(long, short = 'v', action = clap::ArgAction::Count)]
        verbose: u8,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();
    match &args.command {
        Commands::Lex { file, verbose } => {
            lex_source_file(file, *verbose > 0)?;
        }
        Commands::Run { file, verbose } => {
            let tokens = lex_source_file(file, *verbose > 0)?;
            let mut runtime = Runtime::default();
            runtime.run(tokens);

            if *verbose > 0 {
                println!("----------------------");
                println!("");
                println!("Variables in runtime memory after script execution");
                for (k, v) in runtime.variables {
                    println!("{k}: {v:?}");
                }
            }
        }
    }

    Ok(())
}

fn lex_source_file(file: &PathBuf, verbose: bool) -> Result<Vec<Token>> {
    let tokens = Lexer::tokenize(fs::read_to_string(file)?.chars())
        .filter_map(Result::ok)
        .collect();

    if verbose {
        println!("{:?}", tokens);
    }

    Ok(tokens)
}
