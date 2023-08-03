#[cfg(test)]
#[test]
fn generic() {
    use crate::parser::{
        parser::Statement,
        tests::test_helpers::{parse, statement},
    };

    assert_eq!(parse("function a() end"), statement(Statement::FunctionDefinition { identifier: , parameter_list: (), block: () }));
}
