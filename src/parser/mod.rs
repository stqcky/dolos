use crate::parser::ast::definition::Chunk;

use self::ast::definition::Block;

pub mod ast;

pub fn parse(source: &str) -> Result<Chunk, full_moon::Error> {
    let ast = full_moon::parse(source)?;
    let main_block = ast.nodes();

    Ok(Chunk {
        block: Block::from(main_block),
    })
}
