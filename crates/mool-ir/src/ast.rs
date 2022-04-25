use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Program {
    Let(Variable, Expr),
    Expr(Expr),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Expr {
    Variable(Variable),
    Literal(Literal),
    Function(Function),
    Call(String, Vec<Expr>),
    Operator(Operator),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Variable {
    pub name: String,
    pub global: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Function {
    pub args: Vec<FunctionArg>,
    pub rtn: String,
    pub body: Vec<Program>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FunctionArg {
    pub arg: Variable,
    pub annotation: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Operator {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Tensor(Vec<Literal>),
}
