use crate::parser::ast::definition::{
    FunctionCallExpression, IfStatement, TableField, TableIndex, TableMember,
};

use super::definition::{
    AnonFunctionExpression, AssignmentStatement, Block, ElseIf, Expression, FunctionCallStatement,
    FunctionDefinitionStatement, GenericForStatement, LastStatement, LocalDeclarationStatement,
    LocalFunctionDefinitionStatement, NumericForStatement, Parameter, RepeatStatement,
    ReturnStatement, Statement, TableMethod, Variable, WhileStatement,
};

fn get_args(args: &full_moon::ast::FunctionArgs) -> Vec<Expression> {
    match args {
        full_moon::ast::FunctionArgs::Parentheses {
            parentheses,
            arguments,
        } => arguments.iter().map(|x| Expression::from(x)).collect(),
        full_moon::ast::FunctionArgs::String(token) => {
            vec![Expression::LiteralString(token.to_string())]
        }
        full_moon::ast::FunctionArgs::TableConstructor(table) => {
            vec![Expression::from(table)]
        }
        _ => panic!("unexpected FunctionArgs"),
    }
}

fn expression_prefix_suffixes(
    prefix: &full_moon::ast::Prefix,
    suffixes: Vec<&full_moon::ast::Suffix>,
) -> Expression {
    let mut exp = match prefix {
        full_moon::ast::Prefix::Expression(exp) => Expression::from(exp),
        full_moon::ast::Prefix::Name(token) => {
            Expression::Variable(Variable::Identifier(token.to_string()))
        }
        _ => panic!("unexpected Prefix"),
    };

    for suffix in suffixes {
        match suffix {
            full_moon::ast::Suffix::Call(call) => match call {
                full_moon::ast::Call::AnonymousCall(args) => {
                    exp = Expression::FunctionCall(FunctionCallExpression {
                        callee: Box::new(exp),
                        arguments: get_args(&args),
                    });
                }
                full_moon::ast::Call::MethodCall(call) => {
                    let method = call.name().to_string();
                    let args = get_args(call.args());

                    exp = Expression::FunctionCall(FunctionCallExpression {
                        callee: Box::new(Expression::Variable(Variable::TableMethod(
                            TableMethod {
                                base: Box::new(exp),
                                method,
                            },
                        ))),
                        arguments: args,
                    });
                }
                _ => panic!("unexpected Suffix::Call"),
            },
            full_moon::ast::Suffix::Index(idx) => match idx {
                full_moon::ast::Index::Brackets {
                    brackets,
                    expression,
                } => {
                    exp = Expression::Variable(Variable::TableIndex(TableIndex {
                        base: Box::new(exp),
                        index: Box::new(Expression::from(expression)),
                    }));
                }
                full_moon::ast::Index::Dot { dot, name } => {
                    exp = Expression::Variable(Variable::TableMember(TableMember {
                        base: Box::new(exp),
                        member: name.to_string(),
                    }));
                }
                _ => panic!("unexpected Suffix::Index"),
            },
            _ => panic!("unexpected Suffix"),
        }
    }

    exp
}

fn variable_prefix_suffixes(
    prefix: &full_moon::ast::Prefix,
    suffixes: Vec<&full_moon::ast::Suffix>,
) -> Variable {
    let mut var = match prefix {
        full_moon::ast::Prefix::Name(token) => Variable::Identifier(token.to_string()),
        _ => panic!("unexpected Prefix"),
    };

    for suffix in suffixes {
        match suffix {
            full_moon::ast::Suffix::Index(idx) => match idx {
                full_moon::ast::Index::Brackets {
                    brackets,
                    expression,
                } => {
                    // FIXME: this shit is retarded!!!!
                    // it will probably cause issues down the line
                    // maybe create a separate struct for assignment variables?

                    var = Variable::TableIndex(TableIndex {
                        base: Box::new(Expression::Variable(var)),
                        index: Box::new(Expression::from(expression)),
                    });
                }
                full_moon::ast::Index::Dot { dot, name } => {
                    var = Variable::TableMember(TableMember {
                        base: Box::new(Expression::Variable(var)),
                        member: name.to_string(),
                    });
                }
                _ => panic!("unexpected Suffix::Index"),
            },
            _ => panic!("unexpected Suffix"),
        }
    }

    var
}

impl From<full_moon::ast::Block> for Block {
    fn from(value: full_moon::ast::Block) -> Self {
        Block {
            statements: value
                .stmts()
                .map(|stmt| Statement::from(stmt.clone()))
                .collect(),
            last_statement: value.last_stmt().map(|stmt| match stmt {
                full_moon::ast::LastStmt::Break(_) => LastStatement::Break,
                full_moon::ast::LastStmt::Return(ret) => LastStatement::Return(ReturnStatement {
                    expression_list: ret.returns().iter().map(|x| x.into()).collect(),
                }),
                _ => panic!("unexpected LastStmt"),
            }),
        }
    }
}

impl From<&full_moon::ast::Block> for Block {
    fn from(value: &full_moon::ast::Block) -> Self {
        Block::from(value.clone())
    }
}

impl From<full_moon::ast::VarExpression> for Expression {
    fn from(value: full_moon::ast::VarExpression) -> Self {
        expression_prefix_suffixes(value.prefix(), value.suffixes().collect())
    }
}

impl From<&full_moon::ast::VarExpression> for Expression {
    fn from(value: &full_moon::ast::VarExpression) -> Self {
        Expression::from(value.clone())
    }
}

impl From<full_moon::ast::Var> for Variable {
    fn from(value: full_moon::ast::Var) -> Self {
        match value {
            full_moon::ast::Var::Expression(exp) => {
                variable_prefix_suffixes(exp.prefix(), exp.suffixes().collect())
            }
            full_moon::ast::Var::Name(token) => Variable::Identifier(token.to_string()),
            _ => panic!("unexpected Var"),
        }
    }
}

impl From<&full_moon::ast::Var> for Variable {
    fn from(value: &full_moon::ast::Var) -> Self {
        Variable::from(value.clone())
    }
}

impl From<full_moon::ast::ElseIf> for ElseIf {
    fn from(value: full_moon::ast::ElseIf) -> Self {
        ElseIf {
            condition: Expression::from(value.condition().clone()),
            block: Block::from(value.block().clone()),
        }
    }
}

impl From<full_moon::ast::Stmt> for Statement {
    fn from(value: full_moon::ast::Stmt) -> Self {
        match value {
            full_moon::ast::Stmt::Assignment(stmt) => {
                let variables = stmt
                    .variables()
                    .iter()
                    .map(|var| Variable::from(var))
                    .collect();

                let expressions = stmt
                    .expressions()
                    .iter()
                    .map(|exp| Expression::from(exp))
                    .collect();

                Statement::Assignment(AssignmentStatement {
                    variable_list: variables,
                    expression_list: expressions,
                })
            }
            full_moon::ast::Stmt::Do(stmt) => Statement::Scope(Block::from(stmt.block())),
            full_moon::ast::Stmt::FunctionCall(stmt) => {
                let exp = expression_prefix_suffixes(stmt.prefix(), stmt.suffixes().collect());

                match exp {
                    Expression::FunctionCall(call) => {
                        Statement::FunctionCall(FunctionCallStatement {
                            callee: call.callee,
                            arguments: call.arguments
                        })
                    },
                    _ => panic!("unexpected Expression type while translating full_moon::ast::Stmt::FunctionCall, expected Expression::FunctionCall, got: {:#?}", exp)
                }
            }
            full_moon::ast::Stmt::FunctionDeclaration(stmt) => {
                let mut names = stmt.name().names().iter();

                let mut identifier = Variable::Identifier(names.next().expect("unexpected empty names list while translating full_moon::ast::Stmt::FunctionDeclaration").to_string());

                while let Some(name) = names.next() {
                    identifier = Variable::TableMember(TableMember {
                        base: Box::new(Expression::Variable(identifier)),
                        member: name.to_string(),
                    });
                }

                if let Some(method) = stmt.name().method_name() {
                    identifier = Variable::TableMethod(TableMethod {
                        base: Box::new(Expression::Variable(identifier)),
                        method: method.to_string(),
                    })
                }

                let parameter_list = stmt
                    .body()
                    .parameters()
                    .iter()
                    .map(|x| Parameter::from(x))
                    .collect();

                Statement::FunctionDefinition(FunctionDefinitionStatement {
                    identifier,
                    parameter_list,
                    block: Block::from(stmt.body().block()),
                })
            }
            full_moon::ast::Stmt::GenericFor(stmt) => {
                let identifier_list = stmt.names().iter().map(|x| x.to_string()).collect();
                let expression_list = stmt
                    .expressions()
                    .iter()
                    .map(|x| Expression::from(x))
                    .collect();

                Statement::GenericFor(GenericForStatement {
                    identifier_list,
                    expression_list,
                    block: Block::from(stmt.block()),
                })
            }
            full_moon::ast::Stmt::If(stmt) => Statement::If(IfStatement {
                condition: Expression::from(stmt.condition()),
                block: Block::from(stmt.block()),
                elseif_blocks: stmt.else_if().map_or(vec![], |elseif| {
                    elseif
                        .iter()
                        .map(|elseif| ElseIf::from(elseif.clone()))
                        .collect()
                }),
                else_block: stmt.else_block().map(|block| Block::from(block)),
            }),
            full_moon::ast::Stmt::LocalAssignment(stmt) => {
                let identifiers = stmt.names().iter().map(|token| token.to_string()).collect();

                let expressions = stmt
                    .expressions()
                    .iter()
                    .map(|exp| Expression::from(exp))
                    .collect();

                Statement::LocalDeclaration(LocalDeclarationStatement {
                    identifier_list: identifiers,
                    expression_list: expressions,
                })
            }
            full_moon::ast::Stmt::LocalFunction(stmt) => {
                let parameter_list = stmt
                    .body()
                    .parameters()
                    .iter()
                    .map(|x| Parameter::from(x))
                    .collect();

                let block = stmt.body().block().into();

                Statement::LocalFunctionDefinition(LocalFunctionDefinitionStatement {
                    identifier: Variable::Identifier(stmt.name().to_string()),
                    parameter_list,
                    block,
                })
            }
            full_moon::ast::Stmt::NumericFor(stmt) => Statement::NumericFor(NumericForStatement {
                identifier: stmt.index_variable().to_string(),
                start: stmt.start().into(),
                end: stmt.end().into(),
                step: stmt.step().map(|x| x.into()),
                block: stmt.block().into(),
            }),
            full_moon::ast::Stmt::Repeat(stmt) => Statement::Repeat(RepeatStatement {
                block: stmt.block().into(),
                condition: stmt.until().into(),
            }),
            full_moon::ast::Stmt::While(stmt) => Statement::While(WhileStatement {
                condition: stmt.condition().into(),
                block: stmt.block().into(),
            }),
            _ => panic!("unexpected Stmt"),
        }
    }
}

impl From<&full_moon::ast::Stmt> for Statement {
    fn from(value: &full_moon::ast::Stmt) -> Self {
        Statement::from(value.clone())
    }
}

impl From<full_moon::ast::Expression> for Expression {
    fn from(value: full_moon::ast::Expression) -> Self {
        match value {
            full_moon::ast::Expression::BinaryOperator { lhs, binop, rhs } => {
                let lhs = Box::new(Expression::from(lhs));
                let rhs = Box::new(Expression::from(rhs));

                use full_moon::ast::BinOp;

                match binop {
                    BinOp::And(_) => Expression::And(lhs, rhs),
                    BinOp::Caret(_) => Expression::Exponentiation(lhs, rhs),
                    BinOp::GreaterThan(_) => Expression::GreaterThan(lhs, rhs),
                    BinOp::GreaterThanEqual(_) => Expression::GreaterThanOrEqual(lhs, rhs),
                    BinOp::LessThan(_) => Expression::LessThan(lhs, rhs),
                    BinOp::LessThanEqual(_) => Expression::LessThanOrEqual(lhs, rhs),
                    BinOp::Minus(_) => Expression::Subtraction(lhs, rhs),
                    BinOp::Or(_) => Expression::Or(lhs, rhs),
                    BinOp::Percent(_) => Expression::Modulo(lhs, rhs),
                    BinOp::Plus(_) => Expression::Addition(lhs, rhs),
                    BinOp::Slash(_) => Expression::Division(lhs, rhs),
                    BinOp::Star(_) => Expression::Multiplication(lhs, rhs),
                    BinOp::TildeEqual(_) => Expression::NotEqual(lhs, rhs),
                    BinOp::TwoDots(_) => Expression::Concatenation(lhs, rhs),
                    BinOp::TwoEqual(_) => Expression::Equal(lhs, rhs),
                    _ => panic!("unexpected BinOp"),
                }
            }
            full_moon::ast::Expression::Parentheses {
                contained,
                expression,
            } => Expression::Parenthesized(Box::new(Expression::from(expression))),
            full_moon::ast::Expression::UnaryOperator { unop, expression } => {
                let expr = Box::new(Expression::from(expression));

                use full_moon::ast::UnOp;

                match unop {
                    UnOp::Minus(_) => Expression::Negative(expr),
                    UnOp::Not(_) => Expression::Not(expr),
                    UnOp::Hash(_) => Expression::Length(expr),
                    _ => panic!("unexpected UnOp"),
                }
            }
            full_moon::ast::Expression::Value { value } => match *value {
                full_moon::ast::Value::Function((_, func)) => {
                    let params = func
                        .parameters()
                        .iter()
                        .map(|x| Parameter::from(x))
                        .collect();

                    Expression::AnonFunctionDefinition(AnonFunctionExpression {
                        parameter_list: params,
                        block: Block::from(func.block()),
                    })
                }
                full_moon::ast::Value::FunctionCall(call) => {
                    expression_prefix_suffixes(call.prefix(), call.suffixes().collect())
                }
                full_moon::ast::Value::TableConstructor(table) => Expression::from(table),
                full_moon::ast::Value::Number(token) => {
                    Expression::LiteralNumber(token.to_string().trim().parse::<f64>().unwrap())
                }
                full_moon::ast::Value::ParenthesesExpression(expr) => {
                    Expression::Parenthesized(Box::new(Expression::from(expr)))
                }
                full_moon::ast::Value::String(token) => {
                    Expression::LiteralString(token.to_string())
                }
                full_moon::ast::Value::Symbol(token) => match token.token_type() {
                    full_moon::tokenizer::TokenType::Symbol { symbol } => match symbol {
                        full_moon::tokenizer::Symbol::False => Expression::False,
                        full_moon::tokenizer::Symbol::Nil => Expression::Nil,
                        full_moon::tokenizer::Symbol::True => Expression::True,
                        _ => panic!("unexpected symbol: {}", symbol),
                    },
                    _ => panic!(
                        "unexpected Value::Symbol that is not a TokenType::Symbol: {:#?}",
                        token
                    ),
                },
                full_moon::ast::Value::Var(var) => match var {
                    full_moon::ast::Var::Expression(expr) => {
                        expression_prefix_suffixes(expr.prefix(), expr.suffixes().collect())
                    }
                    full_moon::ast::Var::Name(token) => {
                        Expression::Variable(Variable::Identifier(token.to_string()))
                    }
                    _ => panic!("unexpected Var"),
                },
                _ => panic!("unexpected Expression::Value"),
            },
            _ => panic!("unexpected Expression"),
        }
    }
}

impl From<Box<full_moon::ast::Expression>> for Expression {
    fn from(value: Box<full_moon::ast::Expression>) -> Self {
        Expression::from(*value)
    }
}

impl From<&full_moon::ast::Expression> for Expression {
    fn from(value: &full_moon::ast::Expression) -> Self {
        Expression::from(value.clone())
    }
}

impl From<full_moon::ast::Parameter> for Parameter {
    fn from(value: full_moon::ast::Parameter) -> Self {
        match value {
            full_moon::ast::Parameter::Ellipse(_) => Parameter::VariableArg,
            full_moon::ast::Parameter::Name(token) => Parameter::Identifier(token.to_string()),
            _ => panic!("unexpected Parameter"),
        }
    }
}

impl From<&full_moon::ast::Parameter> for Parameter {
    fn from(value: &full_moon::ast::Parameter) -> Self {
        Parameter::from(value.clone())
    }
}

impl From<full_moon::ast::TableConstructor> for Expression {
    fn from(value: full_moon::ast::TableConstructor) -> Self {
        let fields = value.fields().iter();

        Expression::TableConstructor(
            fields
                .map(|field| match field {
                    full_moon::ast::Field::ExpressionKey {
                        brackets,
                        key,
                        equal,
                        value,
                    } => {
                        let index = Expression::from(key);
                        let value = Expression::from(value);

                        TableField::IndexValue(index, value)
                    }
                    full_moon::ast::Field::NameKey { key, equal, value } => {
                        TableField::KeyValue(key.to_string(), Expression::from(value))
                    }
                    full_moon::ast::Field::NoKey(value) => {
                        TableField::Value(Expression::from(value))
                    }
                    _ => panic!("unexpected Field"),
                })
                .collect::<Vec<TableField>>(),
        )
    }
}

impl From<&full_moon::ast::TableConstructor> for Expression {
    fn from(value: &full_moon::ast::TableConstructor) -> Self {
        Expression::from(value.clone())
    }
}
