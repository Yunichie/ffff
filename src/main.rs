use std::{
    fs,
    path::PathBuf
};
use clap::Parser;
use ffff::interpreter::exec::Exec;
use ffff::parser::tokenizer::Tokenizer;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_name = "FILE")]
    file: PathBuf
}

fn main() {
    let cli = Cli::parse();
    let content = fs::read_to_string(cli.file).unwrap();

    let mut tokenizer = Tokenizer::new();
    let mut exec = Exec::new();

    match tokenizer.tokenize(&content) {
        Ok(instructions) => {
            if let Err(e) = exec.execute_loop(&instructions) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}