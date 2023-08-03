use std::fs;

use parser::parser::parse;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

mod parser;

fn main() {
    pretty_env_logger::init();

    let source_code = fs::read_to_string("test.lua").unwrap();

    let chunk = parse(&source_code);

    println!("{:#?}", &chunk);

    // let lexer = Lexer::new(source_code);

    // let tokens = lexer.tokenize();

    // println!("{:?}", tokens);
}
