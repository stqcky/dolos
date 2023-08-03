#[cfg(test)]
#[test]
fn generic() {
    use crate::parser::{
        parser::{Block, ElseIf, Statement},
        tests::test_helpers::{bool, empty_block, identifier, number, parse, statement},
    };

    let a_eq_1 = || Block {
        statements: vec![Statement::Assignment {
            identifier_list: vec![identifier("a")],
            expression_list: vec![number(1.0)],
        }],
        return_statement: None,
    };

    assert_eq!(
        parse("if true then end"),
        statement(Statement::If {
            expression: bool(true),
            block: empty_block(),
            elseif: vec!(),
            else_block: None
        })
    );

    assert_eq!(
        parse("if true a = 1 then end"),
        statement(Statement::If {
            expression: bool(true),
            block: a_eq_1(),
            elseif: vec!(),
            else_block: None
        })
    );

    assert_eq!(
        parse("if false then elseif true then end"),
        statement(Statement::If {
            expression: bool(false),
            block: empty_block(),
            elseif: vec!(ElseIf {
                expression: bool(true),
                block: empty_block()
            }),
            else_block: None
        })
    );

    assert_eq!(
        parse("if false then a = 1 elseif true then a = 1 end"),
        statement(Statement::If {
            expression: bool(false),
            block: a_eq_1(),
            elseif: vec!(ElseIf {
                expression: bool(true),
                block: a_eq_1()
            }),
            else_block: None
        })
    );

    assert_eq!(
        parse("if false then a = 1 elseif true then a = 1 elseif 100 then a = 1 end"),
        statement(Statement::If {
            expression: bool(false),
            block: a_eq_1(),
            elseif: vec!(
                ElseIf {
                    expression: bool(true),
                    block: a_eq_1()
                },
                ElseIf {
                    expression: number(100.0),
                    block: a_eq_1()
                }
            ),
            else_block: None
        })
    );

    assert_eq!(
        parse("if false then a = 1 elseif true then a = 1 elseif 100 then a = 1 else a = 1 end"),
        statement(Statement::If {
            expression: bool(false),
            block: a_eq_1(),
            elseif: vec!(
                ElseIf {
                    expression: bool(true),
                    block: a_eq_1()
                },
                ElseIf {
                    expression: number(100.0),
                    block: a_eq_1()
                }
            ),
            else_block: Some(a_eq_1())
        })
    );

    assert_eq!(
        parse("if true then else end"),
        statement(Statement::If {
            expression: bool(true),
            block: empty_block(),
            elseif: vec!(),
            else_block: Some(empty_block())
        })
    );

    assert_eq!(
        parse("if true then a = 1 else a = 1 end"),
        statement(Statement::If {
            expression: bool(true),
            block: a_eq_1(),
            elseif: vec!(),
            else_block: Some(a_eq_1())
        })
    );
}

#[cfg(test)]
#[test]
fn max_whitespace() {
    use crate::parser::{
        parser::{Block, ElseIf, Statement},
        tests::test_helpers::{bool, empty_block, identifier, number, parse, statement},
    };

    let a_eq_1 = || Block {
        statements: vec![Statement::Assignment {
            identifier_list: vec![identifier("a")],
            expression_list: vec![number(1.0)],
        }],
        return_statement: None,
    };

    assert_eq!(
        parse("   if    true    then    end   "),
        statement(Statement::If {
            expression: bool(true),
            block: empty_block(),
            elseif: vec!(),
            else_block: None
        })
    );

    assert_eq!(
        parse("   if    true    a   =    1    then   end   "),
        statement(Statement::If {
            expression: bool(true),
            block: a_eq_1(),
            elseif: vec!(),
            else_block: None
        })
    );

    assert_eq!(
        parse("   if    false   then    elseif   true    then    end   "),
        statement(Statement::If {
            expression: bool(false),
            block: empty_block(),
            elseif: vec!(ElseIf {
                expression: bool(true),
                block: empty_block()
            }),
            else_block: None
        })
    );

    assert_eq!(
        parse("   if    false    then    a    =    1   elseif   true   then   a   =   1   end  "),
        statement(Statement::If {
            expression: bool(false),
            block: a_eq_1(),
            elseif: vec!(ElseIf {
                expression: bool(true),
                block: a_eq_1()
            }),
            else_block: None
        })
    );

    assert_eq!(
        parse("   if   false   then   a   =   1   elseif   true   then   a   =   1   elseif   100   then   a   =   1   end   "),
        statement(Statement::If {
            expression: bool(false),
            block: a_eq_1(),
            elseif: vec!(
                ElseIf {
                    expression: bool(true),
                    block: a_eq_1()
                },
                ElseIf {
                    expression: number(100.0),
                    block: a_eq_1()
                }
            ),
            else_block: None
        })
    );

    assert_eq!(
        parse("   if   false   then   a   =   1   elseif   true   then   a   =   1   elseif   100   then   a   =   1   else   a   =   1   end   "),
        statement(Statement::If {
            expression: bool(false),
            block: a_eq_1(),
            elseif: vec!(
                ElseIf {
                    expression: bool(true),
                    block: a_eq_1()
                },
                ElseIf {
                    expression: number(100.0),
                    block: a_eq_1()
                }
            ),
            else_block: Some(a_eq_1())
        })
    );

    assert_eq!(
        parse("   if    true    then    else    end   "),
        statement(Statement::If {
            expression: bool(true),
            block: empty_block(),
            elseif: vec!(),
            else_block: Some(empty_block())
        })
    );

    assert_eq!(
        parse("   if    true    then    a    =    1    else    a    =    1    end   "),
        statement(Statement::If {
            expression: bool(true),
            block: a_eq_1(),
            elseif: vec!(),
            else_block: Some(a_eq_1())
        })
    );
}

#[cfg(test)]
#[test]
fn no_whitespace() {
    use crate::parser::parser::parse;

    assert_eq!(parse("iftruethenend").is_err(), true);

    assert_eq!(parse("iftruea=1thenend").is_err(), true);

    assert_eq!(parse("iffalsethenelseiftruethenend").is_err(), true);

    assert_eq!(parse("iffalsethena=1elseiftruethena=1end").is_err(), true);

    assert_eq!(
        parse("iffalsethena=1elseiftruethena=1elseif100thena=1end").is_err(),
        true
    );

    assert_eq!(
        parse("iffalsethena=1elseiftruethena=1elseif100thena=1elsea=1end").is_err(),
        true
    );

    assert_eq!(parse("iftruethenelseend").is_err(), true);

    assert_eq!(parse("iftruethena=1elsea=1end").is_err(), true);
}
