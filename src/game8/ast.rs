use std::rc::Rc;

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
    BinOp(BinOperator, Rc<Expression>, Rc<Expression>),
    FuncCallExpr(String, Vec<Rc<Expression>>),
}

#[derive(Debug)]
pub enum Statement {
    Assign(String, Rc<Expression>),
    IfThen(Rc<Expression>, Vec<Rc<Statement>>),
    IfElse(Rc<Expression>, Vec<Rc<Statement>>, Vec<Rc<Statement>>),
    While(Rc<Expression>, Vec<Rc<Statement>>),
    FuncCallStmt(String, Vec<String>),
    Return(Rc<Expression>),
}

#[derive(Debug)]
pub struct FuncDefinition(pub String, pub Vec<String>, pub Vec<Rc<Statement>>);

#[derive(Debug)]
pub struct Program(pub Vec<FuncDefinition>, pub Vec<Rc<Statement>>);
