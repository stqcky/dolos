#[cfg(test)]
#[test]
fn label() {
    use crate::parser::{
        parser::Statement,
        tests::test_helpers::{identifier, parse, statement},
    };

    assert_eq!(
        parse("::label::"),
        statement(Statement::Label(identifier("label")))
    );

    assert_eq!(
        parse("    ::_cool_label_777::    "),
        statement(Statement::Label(identifier("_cool_label_777")))
    );
}
