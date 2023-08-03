#[cfg(test)]
#[test]
fn generic() {
    use crate::parser::{
        parser::{Expression, Identifier, Statement},
        tests::test_helpers::{empty_block, identifier, number, parse, statement, string},
    };

    assert_eq!(
        parse("func()"),
        statement(Statement::FunctionCall {
            callee: Box::new(Expression::Identifier(identifier("func"))),
            arguments: vec!()
        })
    );

    assert_eq!(
        parse("a.func()"),
        statement(Statement::FunctionCall {
            callee: Box::new(Expression::TableMember {
                base: Box::new(Expression::Identifier(identifier("a"))),
                member: Box::new(identifier("func"))
            }),
            arguments: vec!()
        })
    );

    assert_eq!(
        parse(r#"a["func"]()"#),
        statement(Statement::FunctionCall {
            callee: Box::new(Expression::TableIndex {
                base: Box::new(Expression::Identifier(identifier("a"))),
                index: Box::new(string("func"))
            }),
            arguments: vec!()
        })
    );

    assert_eq!(
        parse("(function() end)()"),
        statement(Statement::FunctionCall {
            callee: Box::new(Expression::AnonFunctionDefinition {
                parameter_list: vec!(),
                block: empty_block()
            }),
            arguments: vec!()
        })
    );

    assert_eq!(
        parse("func(a, 7, function() end)"),
        statement(Statement::FunctionCall {
            callee: Box::new(Expression::Identifier(identifier("func"))),
            arguments: vec!(
                Expression::Identifier(identifier("a")),
                number(7.0),
                Expression::AnonFunctionDefinition {
                    parameter_list: vec!(),
                    block: empty_block()
                }
            )
        })
    );

    assert_eq!(
        parse("func(a, b, ...)"),
        statement(Statement::FunctionCall {
            callee: Box::new(Expression::Identifier(identifier("func"))),
            arguments: vec!(
                Expression::Identifier(identifier("a")),
                Expression::Identifier(identifier("b")),
                Expression::VariableArgument
            )
        })
    );
}

#[cfg(test)]
#[test]
fn max_whitespace() {
    use crate::parser::{
        parser::{Expression, Identifier, Statement},
        tests::test_helpers::{empty_block, identifier, number, parse, statement, string},
    };

    assert_eq!(
        parse("   func   (   )   "),
        statement(Statement::FunctionCall {
            callee: Box::new(Expression::Identifier(identifier("func"))),
            arguments: vec!()
        })
    );

    assert_eq!(
        parse("   a   .   func   (   )   "),
        statement(Statement::FunctionCall {
            callee: Box::new(Expression::TableMember {
                base: Box::new(Expression::Identifier(identifier("a"))),
                member: Box::new(identifier("func"))
            }),
            arguments: vec!()
        })
    );

    assert_eq!(
        parse(r#"   a   ["func"   ]   (   )   "#),
        statement(Statement::FunctionCall {
            callee: Box::new(Expression::TableIndex {
                base: Box::new(Expression::Identifier(identifier("a"))),
                index: Box::new(string("func"))
            }),
            arguments: vec!()
        })
    );

    assert_eq!(
        parse("   (   function   (   )    end   )   (   )   "),
        statement(Statement::FunctionCall {
            callee: Box::new(Expression::AnonFunctionDefinition {
                parameter_list: vec!(),
                block: empty_block()
            }),
            arguments: vec!()
        })
    );

    assert_eq!(
        parse("   func   (   a   ,    7   ,    function   (   )    end   )   "),
        statement(Statement::FunctionCall {
            callee: Box::new(Expression::Identifier(identifier("func"))),
            arguments: vec!(
                Expression::Identifier(identifier("a")),
                number(7.0),
                Expression::AnonFunctionDefinition {
                    parameter_list: vec!(),
                    block: empty_block()
                }
            )
        })
    );

    assert_eq!(
        parse("   func   (   a   ,    b   ,    ...   )   "),
        statement(Statement::FunctionCall {
            callee: Box::new(Expression::Identifier(identifier("func"))),
            arguments: vec!(
                Expression::Identifier(identifier("a")),
                Expression::Identifier(identifier("b")),
                Expression::VariableArgument
            )
        })
    );
}
