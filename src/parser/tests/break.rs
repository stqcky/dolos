#[cfg(test)]
#[test]
fn generic() {
    use crate::parser::{
        parser::Statement,
        tests::test_helpers::{parse, statement},
    };

    assert_eq!(parse("break"), statement(Statement::Break));
}

#[cfg(test)]
#[test]
fn max_whitespace() {
    use crate::parser::{
        parser::Statement,
        tests::test_helpers::{parse, statement},
    };

    assert_eq!(parse("    break    "), statement(Statement::Break));
}
