use peg::{error::ParseError, str::LineCol};
use log::info;

#[derive(Clone, PartialEq, Debug)]
pub struct Chunk {
    block: Block,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Block {
    statements: Vec<Statement>,
    return_statement: Option<ReturnStatement>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ReturnStatement {}

#[derive(Clone, PartialEq, Debug)]
pub enum Statement {
    Semicolon,
    LocalDeclaration {
        identifier_list: Vec<Identifier>,
        expression_list: Vec<Expression>,
    },
    FunctionCall(Expression),
    Assignment {
        identifier_list: Vec<Identifier>,
        expression_list: Vec<Expression>
    },
    Label(Identifier),
    Break,
    Goto(Identifier),
    Scope(Block),
    While {
        expression: Expression,
        block: Block
    },
    Repeat {
        block: Block,
        expression: Expression
    },
    If {
        expression: Expression,
        block: Block,
        elseif: Vec<ElseIf>,
        else_block: Option<Block>
    },
    NumericFor {
        identifier: Identifier,
        start: Expression,
        end: Expression,
        step: Expression
    },
    GenericFor,
    FunctionDefinition,
    LocalFunctionDefinition,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ElseIf {
    expression: Expression,
    block: Block
}

#[derive(Clone, PartialEq, Debug)]
pub enum Identifier {
    Variable(String),
    IndexExpression(Vec<String>),
}

#[derive(Clone, PartialEq, Debug)]
pub enum TableField {
    Value(Expression),
    KeyValue(Expression, Expression),
}

#[derive(Clone, PartialEq, Debug)]
pub enum TableFieldSeparator {
    Semicolon,
    Comma,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Expression {
    Number(f64),
    String(String),
    Identifier(Identifier),
    True,
    False,
    Nil,
    TableConstructor(Vec<TableField>),
    FunctionCall {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },

    Parenthesized(Box<Expression>),

    Exponentiation(Box<Expression>, Box<Expression>),

    Not(Box<Expression>),
    Negative(Box<Expression>),

    Multiplication(Box<Expression>, Box<Expression>),
    Division(Box<Expression>, Box<Expression>),
    Modulo(Box<Expression>, Box<Expression>),

    Addition(Box<Expression>, Box<Expression>),
    Subtraction(Box<Expression>, Box<Expression>),

    Concatenation(Box<Expression>, Box<Expression>),

    LessThan(Box<Expression>, Box<Expression>),
    GreaterThan(Box<Expression>, Box<Expression>),
    LessThanOrEqual(Box<Expression>, Box<Expression>),
    GreaterThanOrEqual(Box<Expression>, Box<Expression>),
    NotEqual(Box<Expression>, Box<Expression>),
    Equal(Box<Expression>, Box<Expression>),

    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),

    Length(Box<Expression>),
}

peg::parser! {
    grammar lua() for str {
        rule _ = (" " / "\n")*
        rule __ = quiet!{(" " / "\n")+} / expected!("forced whitespace")

        pub rule chunk() -> Chunk = b:block() {
            Chunk {
                block: b
            }
        }

        rule block() -> Block = _ statements:statement()* _ return_statement:return_statement()? _ {
            Block {
                statements,
                return_statement
            }
        }

        rule statement() -> Statement = stmt:(
            semicolon() 
            / local_declaration() 
            / assignment()
            / label()
            / break()
            / goto()
            / scope()
            / while()
            / if()
            / numeric_for()
            / function_call_statement() 
        ) {
            stmt
        }

        // rule function_definition() -> Statement = _ "function" __ identifier:identifier() _ "(" _ 



        rule numeric_for() -> Statement = _ "for" __ identifier:identifier() _ "=" _ start:expression() _ "," _ end:expression() _ step:numeric_for_step()? _ "do" _ block:block() _ "end" _ {
            
            Statement::NumericFor { identifier, start, end, step: match step {
                Some(v) => v,
                None => Expression::Number(1.0)
            } }
        }

        rule numeric_for_step() -> Expression = _ "," _ step:expression() _ {
            step
        }

        rule if() -> Statement = _ "if" _ expression:expression() _ "then" _ block:block() _ elseif:elseif()* _ else_block:else_block()? _ "end" _ {
            Statement::If { expression, block, elseif, else_block }
        }

        rule else_block() -> Block = _ "else" _ block:block() _ {
            block
        }

        rule elseif() -> ElseIf = _ "elseif" _ expression:expression() _ "then" _ block:block() _ {
            ElseIf { expression, block }
        }

        rule while() -> Statement = _ "while" _ exp:expression() _ "do" _ block:block() _ "end" {
            Statement::While { expression: exp, block }
        }

        rule scope() -> Statement = _ "do" _ block:block() _ "end" _ {
            Statement::Scope(block)
        }

        rule goto() -> Statement = _ "goto" _ label:identifier() _ {
            Statement::Goto(label)
        }

        rule break() -> Statement = _ "break" _ {
            Statement::Break
        }

        rule label() -> Statement = _ "::" _ ident:identifier() _ "::" _ {
            Statement::Label(ident)
        }

        rule assignment() -> Statement = _ vars:identifier_list() _ "=" _ exps:expression_list() _ {
            Statement::Assignment { identifier_list: vars, expression_list: exps }
        }

        rule function_call_statement() -> Statement = _ call:function_call() _ {
            Statement::FunctionCall(call)
        }

        #[cache_left_rec]
        pub rule function_call() -> Expression = _ callee:expression() _ arguments:function_arguments() _ {
            match callee {
                Expression::Parenthesized(_) | Expression::Identifier(_) => Expression::FunctionCall { callee: Box::new(callee), arguments },
                _ => panic!("attempt to call uncallable expression: {:?}", callee)
            }
        }

        rule function_arguments() -> Vec<Expression> = quiet!{_ "(" _ exp_list:expression_list()? _ ")" _ {
            match exp_list {
                Some(v) => v,
                None => vec!()
            }
        }
        / _ table:table_constructor() _ {
            vec![table]
        }
        / _ s:string() _ {
            vec![s]
        }} / expected!("function arguments")

        rule table_constructor() -> Expression = _ "{" _ fields:table_field_list() _ "}" _ {
            Expression::TableConstructor(fields)
        }

        rule table_field_list() -> Vec<TableField> = _ fields:((table_field_key_value() / table_field_identifier_value() / table_field_value()) ** (";" / ",")) {
            fields
        }

        rule table_field_key_value() -> TableField = _ "[" _ key:expression() _ "]" _ "=" _ value:expression() {
            TableField::KeyValue(key, value)
        }

        rule table_field_identifier_value() -> TableField = _ key:identifier() _ "=" value:expression() _ {
            TableField::KeyValue(Expression::Identifier(key), value)
        }

        rule table_field_value() -> TableField = _ value:expression() _ {
            TableField::Value(value)
        }

        rule local_declaration() -> Statement = quiet!{_ "local" _ idents:identifier_list() _ "=" _ exps:expression_list() _ {
            Statement::LocalDeclaration { identifier_list: idents, expression_list: exps }
        }} / expected!("local declaration")

        rule semicolon() -> Statement = ";" {
            Statement::Semicolon
        }

        rule return_statement() -> ReturnStatement = "return" {
            ReturnStatement {}
        }

        rule number() -> Expression = quiet!{n:$(['0'..='9']+) {
            Expression::Number(n.parse().unwrap())
        }} / expected!("number")

        rule string() -> Expression = quiet!{"\"" _ str:$((['a'..='z'] / ['A'..='Z'] / ['0'..='9'])*) _ "\"" {
            Expression::String(str.to_string())
        }} / expected!("string")

        rule identifier() -> Identifier = quiet!{ident:$((['a'..='z'] / ['A'..='Z'] / "_")+ (['a'..='z'] / ['A'..='Z'] / ['0'..='9'])*) {
            Identifier::Variable(ident.to_string())
        }} / expected!("identifier")

        rule identifier_list() -> Vec<Identifier> = quiet!{identifiers:(identifier() ++ (_ "," _)) {
            identifiers
        }} / expected!("identifier list")

        rule expression_list() -> Vec<Expression> = quiet!{expressions:(expression() ++ (_ "," _)) {
            expressions
        }} / expected!("expression list")

        rule boolean() -> Expression = b:$("true" / "false") {
            match b {
                "true" => Expression::True,
                "false" => Expression::False,
                _ => unreachable!("matched unreachable boolean")
            }
        }

        rule nil() -> Expression = "nil" {
            Expression::Nil
        }

        #[cache_left_rec]
        pub rule expression() -> Expression = quiet!{precedence!{
            lhs:(@) _ "or" _ rhs:@ {
                Expression::Or(Box::new(lhs), Box::new(rhs))
            }
            --
            lhs:(@) _ "and" _ rhs:@ {
                Expression::And(Box::new(lhs), Box::new(rhs))
            }
            --
            lhs:(@) _ "==" _ rhs:@ {
                Expression::Equal(Box::new(lhs), Box::new(rhs))
            }
            lhs:(@) _ "~=" _ rhs:@ {
                Expression::NotEqual(Box::new(lhs), Box::new(rhs))
            }
            lhs:(@) _ ">=" _ rhs:@ {
                Expression::GreaterThanOrEqual(Box::new(lhs), Box::new(rhs))
            }
            lhs:(@) _ "<=" _ rhs:@ {
                Expression::LessThanOrEqual(Box::new(lhs), Box::new(rhs))
            }
            lhs:(@) _ ">" _ rhs:@ {
                Expression::GreaterThan(Box::new(lhs), Box::new(rhs))
            }
            lhs:(@) _ "<" _ rhs:@ {
                Expression::LessThan(Box::new(lhs), Box::new(rhs))
            }
            --
            lhs:@ _ ".." _ rhs:(@) {
                Expression::Concatenation(Box::new(lhs), Box::new(rhs))
            }
            --
            lhs:(@) _ "+" _ rhs:@ {
                match (lhs.clone(), rhs.clone()) {
                    (Expression::Number(lhs), Expression::Number(rhs)) => Expression::Number(lhs + rhs),
                    _ => Expression::Addition(Box::new(lhs), Box::new(rhs))
                }
            }
            lhs:(@) _ "-" _ rhs:@ {
                Expression::Subtraction(Box::new(lhs), Box::new(rhs))
            }
            --
            lhs:(@) _ "*" _ rhs:@ {
                Expression::Multiplication(Box::new(lhs), Box::new(rhs))
            }
            lhs:(@) _ "/" _ rhs:@ {
                Expression::Division(Box::new(lhs), Box::new(rhs))
            }
            lhs:(@) _ "%" _ rhs:@ {
                Expression::Modulo(Box::new(lhs), Box::new(rhs))
            }
            --
            "-" _ rhs:@ {
                Expression::Negative(Box::new(rhs))
            }
            "not" _ rhs:@ {
                Expression::Not(Box::new(rhs))
            }
            "#" _ rhs:@ {
                Expression::Length(Box::new(rhs))
            }
            --
            lhs:@ _ "^" _ rhs:(@) {
                Expression::Exponentiation(Box::new(lhs), Box::new(rhs))
            }
            --
            // call:function_call() { call }
            n:number() { n }
            nil:nil() { nil }
            b:boolean() { b }
            s:string() { s }
            ident:identifier() { Expression::Identifier(ident) }
            --
            lparen() _ e:expression() _ rparen() { Expression::Parenthesized(Box::new(e)) }
        }} / expected!("expression")

        rule lparen() = "("
        rule rparen() = ")"
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum TestExpression {
    FunctionCall {
        callee: Box<TestExpression>,
        arguments: Vec<TestExpression>
    },
    Identifier(String)
}

peg::parser! {
    grammar test() for str {
        pub rule identifier() -> TestExpression = i:$(['a'..='z']+) {
            TestExpression::Identifier(i.to_string())
        }

        #[cache_left_rec]
        pub rule expression() -> TestExpression = exp:(function_call() / identifier()) {
            exp
        }
         
        rule lparen() = "("
        rule rparen() = ")"

        #[cache_left_rec]
        pub rule function_call() -> TestExpression = callee:expression() lparen() arg:(expression() ** ",") rparen() {
            TestExpression::FunctionCall { callee: Box::new(callee), arguments: arg }
        }
    }
}


pub fn parse(code: &str) -> Result<Chunk, ParseError<LineCol>> {
    info!("parsing");

    // let input = "abc()";
    // let a = test::expression(&input);
    // println!("{:?}", a);

    lua::chunk(code)
}
