mod lexer;
mod parser;
mod token_stream;
mod graphviz;

use std::fs;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: [invocation] filename")
    }
    else {
        let contents: String = fs::read_to_string(&args[1]).expect("Could not open file");
        let tokens = lexer::lex_string(contents);
        for token in &tokens {
            println!("{:?}", token)
        }
        let _ = parser::parse_stream(&tokens);
    }
}
