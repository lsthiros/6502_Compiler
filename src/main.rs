mod lexer;

fn main() {
    println!("Hello, world!");
    lexer::lex_string(String::from("Hello"))
}
