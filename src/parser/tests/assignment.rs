#[cfg(test)]
#[test]
fn generic() {
    use crate::parser::{
        parser::Statement,
        tests::test_helpers::{identifier, number, parse, statement},
    };

    assert_eq!(
        parse("B = 1"),
        statement(Statement::Assignment {
            identifier_list: vec!(identifier("B")),
            expression_list: vec!(number(1.0))
        })
    );

    assert_eq!(
        parse("b, c = 10, 20"),
        statement(Statement::Assignment {
            identifier_list: vec!(identifier("b"), identifier("c")),
            expression_list: vec!(number(10.0), number(20.0))
        })
    );
}

#[cfg(test)]
#[test]
fn max_whitespace() {
    use crate::parser::{
        parser::Statement,
        tests::test_helpers::{identifier, number, parse, statement},
    };

    assert_eq!(
        parse("    B    =    1    "),
        statement(Statement::Assignment {
            identifier_list: vec!(identifier("B")),
            expression_list: vec!(number(1.0))
        })
    );

    assert_eq!(
        parse("   b   ,    c    =    10   ,    20   "),
        statement(Statement::Assignment {
            identifier_list: vec!(identifier("b"), identifier("c")),
            expression_list: vec!(number(10.0), number(20.0))
        })
    );
}

#[cfg(test)]
#[test]
fn no_whitespace() {
    use crate::parser::{
        parser::Statement,
        tests::test_helpers::{identifier, number, parse, statement},
    };

    assert_eq!(
        parse("B=1"),
        statement(Statement::Assignment {
            identifier_list: vec!(identifier("B")),
            expression_list: vec!(number(1.0))
        })
    );

    assert_eq!(
        parse("b,c=10,20"),
        statement(Statement::Assignment {
            identifier_list: vec!(identifier("b"), identifier("c")),
            expression_list: vec!(number(10.0), number(20.0))
        })
    );
}
