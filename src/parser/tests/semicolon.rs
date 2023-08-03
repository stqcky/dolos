#[cfg(test)]
#[test]
fn semicolon() {
    use crate::parser::{
        parser::Statement,
        tests::test_helpers::{parse, statement},
    };

    assert_eq!(parse(";"), statement(Statement::Semicolon));
    assert_eq!(parse("   ;   "), statement(Statement::Semicolon));
}
