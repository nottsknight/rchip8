#[derive(Debug)]
pub enum BinOperator {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Xor,
}

#[derive(Debug)]
pub enum UnOperator {
    Neg,
    Not,
}

#[derive(Debug)]
pub enum Expr {
    Number(u16),
    Identifier(String),
    UnOp(UnOperator, Box<Expr>),
    BinOp(BinOperator, Box<Expr>, Box<Expr>),
    ReadDelay,
    ReadKey,
}

#[derive(Debug)]
pub enum Cmd {
    If(Box<Expr>, Box<Cmd>),
    IfElse(Box<Expr>, Box<Cmd>, Box<Cmd>),
    While(Box<Expr>, Box<Cmd>),
    Return(Box<Expr>),
}
