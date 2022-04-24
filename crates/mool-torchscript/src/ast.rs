use serde::{Deserialize, Serialize};

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
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Expr {
    Identifier(String),
    Literal(Literal),
    Operator(Operator),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Statement {
    Expr(Expr),
    Return(Expr)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FunctionArg {
    pub arg: String,
    pub annotation: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FunctionReturn {
    pub rt: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Program {
    Function(String, Vec<FunctionArg>, FunctionReturn, Vec<Statement>),
}
