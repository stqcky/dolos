#[derive(Clone, PartialEq, Debug)]
pub struct Chunk {
    pub block: Block,
}

pub type Identifier = String;

#[derive(Clone, PartialEq, Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub last_statement: Option<LastStatement>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct LocalDeclarationStatement {
    pub identifier_list: Vec<Identifier>,
    pub expression_list: Vec<Expression>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct FunctionCallStatement {
    pub callee: Box<Expression>,
    pub arguments: Vec<Expression>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct AssignmentStatement {
    pub variable_list: Vec<Variable>,
    pub expression_list: Vec<Expression>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct LabelStatement {
    pub identifier: Identifier,
}

#[derive(Clone, PartialEq, Debug)]
pub struct GotoStatement {
    pub label: Identifier,
}

#[derive(Clone, PartialEq, Debug)]
pub struct WhileStatement {
    pub condition: Expression,
    pub block: Block,
}

#[derive(Clone, PartialEq, Debug)]
pub struct RepeatStatement {
    pub block: Block,
    pub condition: Expression,
}

#[derive(Clone, PartialEq, Debug)]
pub struct IfStatement {
    pub condition: Expression,
    pub block: Block,
    pub elseif_blocks: Vec<ElseIf>,
    pub else_block: Option<Block>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ElseIf {
    pub condition: Expression,
    pub block: Block,
}

#[derive(Clone, PartialEq, Debug)]
pub struct NumericForStatement {
    pub identifier: Identifier,
    pub start: Expression,
    pub end: Expression,
    pub step: Option<Expression>,
    pub block: Block,
}

#[derive(Clone, PartialEq, Debug)]
pub struct GenericForStatement {
    pub identifier_list: Vec<Identifier>,
    pub expression_list: Vec<Expression>,
    pub block: Block,
}

#[derive(Clone, PartialEq, Debug)]
pub struct FunctionDefinitionStatement {
    pub identifier: Variable,
    pub parameter_list: Vec<Parameter>,
    pub block: Block,
}

#[derive(Clone, PartialEq, Debug)]
pub struct LocalFunctionDefinitionStatement {
    pub identifier: Variable,
    pub parameter_list: Vec<Parameter>,
    pub block: Block,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ReturnStatement {
    pub expression_list: Vec<Expression>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum LastStatement {
    Break,
    Return(ReturnStatement),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Statement {
    Semicolon,
    LocalDeclaration(LocalDeclarationStatement),
    FunctionCall(FunctionCallStatement),
    Assignment(AssignmentStatement),
    Label(Identifier),
    Break,
    Goto(Identifier),
    Scope(Block),
    While(WhileStatement),
    Repeat(RepeatStatement),
    If(IfStatement),
    NumericFor(NumericForStatement),
    GenericFor(GenericForStatement),
    FunctionDefinition(FunctionDefinitionStatement),
    LocalFunctionDefinition(LocalFunctionDefinitionStatement),
}

#[derive(Clone, PartialEq, Debug)]
pub enum TableField {
    Value(Expression),
    IndexValue(Expression, Expression),
    KeyValue(Identifier, Expression),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Parameter {
    Identifier(Identifier),
    VariableArg,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TableIndex {
    pub base: Box<Expression>,
    pub index: Box<Expression>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TableMember {
    pub base: Box<Expression>,
    pub member: Identifier,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TableMethod {
    pub base: Box<Expression>,
    pub method: Identifier,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Variable {
    Identifier(Identifier),
    TableIndex(TableIndex),
    TableMember(TableMember),
    TableMethod(TableMethod),
}

#[derive(Clone, PartialEq, Debug)]
pub struct FunctionCallExpression {
    pub callee: Box<Expression>,
    pub arguments: Vec<Expression>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct AnonFunctionExpression {
    pub parameter_list: Vec<Parameter>,
    pub block: Block,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Expression {
    LiteralNumber(f64),
    LiteralString(String),
    True,
    False,
    Nil,
    TableConstructor(Vec<TableField>),
    FunctionCall(FunctionCallExpression),
    AnonFunctionDefinition(AnonFunctionExpression),
    Variable(Variable),
    VariableArgument,

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
