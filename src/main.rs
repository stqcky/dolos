use std::fs;

extern crate log;
extern crate pretty_env_logger;

mod cfg;
mod parser;
mod vm;

fn main() {
    pretty_env_logger::init();

    let source_code = fs::read_to_string("test.lua").unwrap();

    let ast = parser::parse(&source_code).unwrap();
    let cfg = cfg::translator::translate(&ast.block);
    cfg::visualization::visualize(cfg);
}
