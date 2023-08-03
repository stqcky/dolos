#[cfg(test)]
#[test]
fn goto() {
    use crate::parser::{
        parser::Statement,
        tests::test_helpers::{identifier, parse, statement},
    };

    assert_eq!(
        parse("goto label"),
        statement(Statement::Goto(identifier("label")))
    );
    assert_eq!(
        parse("    goto     _label2   "),
        statement(Statement::Goto(identifier("_label2")))
    );
}
