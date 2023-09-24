use std::fmt::Display;

use super::definition::{
    Block, Chunk, ElseIf, Expression, LastStatement, Parameter, Statement, TableField, Variable,
};

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.block)
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let statements = self
            .statements
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<String>>()
            .join("\n");

        let last = self
            .last_statement
            .clone()
            .map_or("".to_string(), |last| format!("{last}"));

        write!(f, "{statements} {last}")
    }
}

impl Display for LastStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LastStatement::Break => write!(f, "break"),
            LastStatement::Return(stmt) => write!(f, "return {}", join(&stmt.expression_list, ",")),
        }
    }
}

fn join<T>(list: &Vec<T>, sep: &str) -> String
where
    T: std::fmt::Display,
{
    list.iter()
        .map(|x| format!("{}", x))
        .collect::<Vec<String>>()
        .join(sep)
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Semicolon => write!(f, ";"),
            Statement::LocalDeclaration(stmt) => {
                let identifiers = stmt.identifier_list.join(",");
                let expressions = join(&stmt.expression_list, ",");

                write!(f, "local {identifiers} = {expressions}")
            }
            Statement::FunctionCall(stmt) => {
                let callee = &stmt.callee;
                let args = join(&stmt.arguments, ",");

                write!(f, "{callee}({args})")
            }
            Statement::Assignment(stmt) => {
                let identifiers = join(&stmt.variable_list, ",");
                let expressions = join(&stmt.expression_list, ",");

                write!(f, "{identifiers} = {expressions}")
            }
            Statement::Label(identifier) => write!(f, "::{identifier}::"),
            Statement::Break => write!(f, "break"),
            Statement::Goto(identifier) => write!(f, "goto {identifier}"),
            Statement::Scope(block) => write!(f, "do {block} end"),
            Statement::While(stmt) => {
                let condition = &stmt.condition;
                let block = &stmt.block;

                write!(f, "while {condition} do {block} end")
            }
            Statement::Repeat(stmt) => {
                let condition = &stmt.condition;
                let block = &stmt.block;

                write!(f, "repeat {block} until {condition}")
            }
            Statement::If(stmt) => {
                let condition = &stmt.condition;
                let block = &stmt.block;

                let else_ifs = join(&stmt.elseif_blocks, "\n");

                let else_str = stmt
                    .else_block
                    .clone()
                    .map_or("".to_string(), |block| format!("else {block}"));

                write!(f, "if {condition} then {block} {else_ifs} {else_str} end")
            }
            Statement::NumericFor(stmt) => {
                let identifier = &stmt.identifier;
                let start = &stmt.start;
                let end = &stmt.end;
                let step = stmt
                    .step
                    .clone()
                    .map_or("".to_string(), |x| format!("{}", x));
                let block = &stmt.block;

                write!(
                    f,
                    "for {identifier} = {start}, {end}, {step} do {block} end"
                )
            }
            Statement::GenericFor(stmt) => {
                let identifiers = join(&stmt.identifier_list, ",");
                let expressions = join(&stmt.expression_list, ",");
                let block = &stmt.block;

                write!(f, "for {identifiers} in {expressions} do {block} end")
            }
            Statement::FunctionDefinition(stmt) => {
                let identifier = &stmt.identifier;
                let parameters = join(&stmt.parameter_list, ",");
                let block = &stmt.block;

                write!(f, "function {identifier}({parameters}) {block} end")
            }
            Statement::LocalFunctionDefinition(stmt) => {
                let identifier = &stmt.identifier;
                let parameters = join(&stmt.parameter_list, ",");
                let block = &stmt.block;

                write!(f, "local function {identifier}({parameters}) {block} end")
            }
        }
    }
}

impl Display for ElseIf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "elseif {} then {}", self.condition, self.block)
    }
}

impl Display for TableField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableField::Value(expr) => write!(f, "{}", expr)?,
            TableField::IndexValue(index, value) => write!(f, "[{}] = {}", index, value)?,
            TableField::KeyValue(key, value) => write!(f, "{} = {}", key, value)?,
        }

        Ok(())
    }
}

impl Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Parameter::Identifier(ident) => write!(f, "{}", ident),
            Parameter::VariableArg => write!(f, "..."),
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LiteralNumber(number) => write!(f, "{}", number),
            Self::LiteralString(string) => write!(f, "{}", string),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::Nil => write!(f, "nil"),
            Expression::TableConstructor(fields) => {
                let fields = join(fields, ",");

                write!(f, "{{{fields}}}")
            }
            Expression::FunctionCall(call) => {
                let callee = &call.callee;
                let args = join(&call.arguments, ",");

                write!(f, "{callee}({args})")
            }
            Expression::AnonFunctionDefinition(func) => {
                let params = join(&func.parameter_list, ",");
                let block = &func.block;

                write!(f, "function({params}) {block} end")
            }
            Expression::VariableArgument => write!(f, "..."),
            Expression::Parenthesized(exp) => write!(f, "({exp})"),
            Expression::Exponentiation(a, b) => write!(f, "{a} ^ {b}"),
            Expression::Not(exp) => write!(f, "~{exp}"),
            Expression::Negative(exp) => write!(f, "-{exp}"),
            Expression::Multiplication(a, b) => write!(f, "{a} * {b}"),
            Expression::Division(a, b) => write!(f, "{a} / {b}"),
            Expression::Modulo(a, b) => write!(f, "{a} % {b}"),
            Expression::Addition(a, b) => write!(f, "{a} + {b}"),
            Expression::Subtraction(a, b) => write!(f, "{a} - {b}"),
            Expression::Concatenation(a, b) => write!(f, "{a} .. {b}"),
            Expression::LessThan(a, b) => write!(f, "{a} < {b}"),
            Expression::GreaterThan(a, b) => write!(f, "{a} > {b}"),
            Expression::LessThanOrEqual(a, b) => write!(f, "{a} <= {b}"),
            Expression::GreaterThanOrEqual(a, b) => write!(f, "{a} >= {b}"),
            Expression::NotEqual(a, b) => write!(f, "{a} ~= {b}"),
            Expression::Equal(a, b) => write!(f, "{a} == {b}"),
            Expression::And(a, b) => write!(f, "{a} and {b}"),
            Expression::Or(a, b) => write!(f, "{a} or {b}"),
            Expression::Length(exp) => write!(f, "#{exp}"),
            Expression::Variable(var) => write!(f, "{var}"),
        }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variable::Identifier(identifier) => write!(f, "{identifier}"),
            Variable::TableIndex(table_index) => {
                write!(f, "{}[{}]", table_index.base, table_index.index)
            }
            Variable::TableMember(table_member) => {
                write!(f, "{}.{}", table_member.base, table_member.member)
            }
            Variable::TableMethod(table_method) => {
                write!(f, "{}:{}", table_method.base, table_method.method)
            }
        }
    }
}
