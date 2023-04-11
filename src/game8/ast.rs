#[derive(Debug)]
pub enum BinOperator {
    ADD,
    SUB,
    MUL,
    DIV,
    EQL,
    NEQ,
    LT,
    LTE,
    GT,
    GTE,
}

#[derive(Debug)]
pub enum Expression {
    Number(u16),
    Variable(String),
    BinOp(BinOperator, Box<Expression>, Box<Expression>),
    FuncCallExpr(String, Vec<Box<Expression>>),
}

#[derive(Debug)]
pub enum Statement {
    Assign(String, Box<Expression>),
    IfThen(Box<Expression>, Vec<Box<Statement>>),
    IfElse(Box<Expression>, Vec<Box<Statement>>, Vec<Box<Statement>>),
    While(Box<Expression>, Vec<Box<Statement>>),
    FuncCallStmt(String, Vec<String>),
    Return(Box<Expression>),
}

#[derive(Debug)]
pub struct FuncDefinition(pub String, pub Vec<String>, pub Vec<Box<Statement>>);

#[derive(Debug)]
pub struct Program(pub Vec<FuncDefinition>, pub Vec<Box<Statement>>);
