#[cfg(test)]
#[test]
fn r#while() {
    use crate::parser::{tests::test_helpers::{parse, statement, number, identifier}, parser::{Statement, Expression, Block, self}};

    assert_eq!(
        parse("while a < 10 do end"),
        statement(Statement::While {
            expression: Expression::LessThan(
                Box::new(Expression::Identifier(identifier("a"))),
                Box::new(number(10.0))
            ),
            block: Block {
                statements: vec!(),
                return_statement: None
            }
        })
    );

    assert_eq!(
        parse("    while    a   <    10    do   end"),
        statement(Statement::While {
            expression: Expression::LessThan(
                Box::new(Expression::Identifier(identifier("a"))),
                Box::new(number(10.0))
            ),
            block: Block {
                statements: vec!(),
                return_statement: None
            }
        })
    );

    assert_eq!(parser::parse("whilea<10do end").is_err(), true);

    assert_eq!(parser::parse("whilea<10doend").is_err(), true);

    assert_eq!(parser::parse("while a<10do end").is_err(), true);
}
