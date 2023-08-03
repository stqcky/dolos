mod assignment;
mod r#break;
mod goto;
mod label;
mod local_declaration;
mod scope;
mod semicolon;
mod r#while;
mod r#if;
mod generic_for;
mod numeric_for;
mod repeat;
mod function_definition;
mod local_function_definiton;
mod function_call_statement;

#[cfg(test)]
pub mod test_helpers {
    use crate::parser::parser::{self, Block, Chunk, Expression, Identifier, Statement};

    pub fn statement(stmt: Statement) -> Chunk {
        Chunk {
            block: Block {
                statements: vec![stmt],
                return_statement: None,
            },
        }
    }

    pub fn number(num: f64) -> Expression {
        Expression::Number(num)
    }

    pub fn string(string: &str) -> Expression {
        Expression::String(String::from(string))
    }

    pub fn bool(b: bool) -> Expression {
        match b {
            true => Expression::True,
            false => Expression::False
        }
    }

    pub fn empty_block() -> Block {
        Block { statements: vec!(), return_statement: None }
    }

    pub fn identifier(name: &str) -> Identifier {
        Identifier {
            name: String::from(name),
        }
    }

    pub fn parse(lua: &str) -> Chunk {
        parser::parse(lua).unwrap()
    }
}
