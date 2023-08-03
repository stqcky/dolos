#[cfg(test)]
#[test]
fn local_declaration() {
    use crate::parser::{
        parser::Statement,
        tests::test_helpers::{identifier, number, parse, statement, string},
    };

    assert_eq!(
        parse("local a = 5"),
        statement(Statement::LocalDeclaration {
            identifier_list: vec!(identifier("a")),
            expression_list: vec!(number(5.0))
        })
    );

    assert_eq!(
        parse("   local     b    =    \"coolstring\"   "),
        statement(Statement::LocalDeclaration {
            identifier_list: vec!(identifier("b")),
            expression_list: vec!(string("coolstring"))
        })
    );

    assert_eq!(
        parse("   local     aa   ,   bb    =    500, 600   "),
        statement(Statement::LocalDeclaration {
            identifier_list: vec!(identifier("aa"), identifier("bb")),
            expression_list: vec!(number(500.0), number(600.0))
        })
    );

    assert_eq!(
        parse("local a=2"),
        statement(Statement::LocalDeclaration {
            identifier_list: vec!(identifier("a")),
            expression_list: vec!(number(2.0))
        })
    )
}
