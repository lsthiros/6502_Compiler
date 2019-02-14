mod lexer;
mod parser;

use std::fs;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: [invocation] filename")
    }
    else {
        let contents: String = fs::read_to_string(&args[1]).expect("Could not open file");
        for token in lexer::lex_string(contents) {
            println!("{:?}", token)
        }
    }
}
