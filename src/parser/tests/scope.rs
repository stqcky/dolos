#[cfg(test)]
#[test]
fn scope() {
    use crate::parser::{
        parser::{Block, Statement},
        tests::test_helpers::{identifier, number, parse, statement},
    };

    assert_eq!(
        parse("do end"),
        statement(Statement::Scope(Block {
            statements: vec!(),
            return_statement: None
        }))
    );

    assert_eq!(
        parse("    do    end    "),
        statement(Statement::Scope(Block {
            statements: vec!(),
            return_statement: None
        }))
    );

    assert_eq!(
        parse(
            r#"    
                do   
                    local a = 5
                end    
                "#
        ),
        statement(Statement::Scope(Block {
            statements: vec!(Statement::LocalDeclaration {
                identifier_list: vec!(identifier("a")),
                expression_list: vec!(number(5.0))
            }),
            return_statement: None
        }))
    );
}
