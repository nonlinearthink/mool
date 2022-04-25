use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Program {
    Function(Function),
    Statement(Expr),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<FunctionArg>,
    pub rtn: String,
    pub body: Vec<FunctionStatement>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FunctionArg {
    pub name: String,
    pub annotation: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FunctionStatement {
    Expr(Expr),
    Return(Expr),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Expr {
    Identifier(String),
    Literal(Literal),
    Operator(Operator),
    Call(String, Vec<Expr>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Operator {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Tensor(Vec<Literal>)
}
