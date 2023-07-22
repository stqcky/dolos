use std::fs;

use lexer::lexer::Lexer;

mod lexer;

fn main() {
    let source_code = fs::read_to_string("test.lua").unwrap();
    let lexer = Lexer::new(source_code);

    let tokens = lexer.tokenize();

    println!("{:?}", tokens);
}
