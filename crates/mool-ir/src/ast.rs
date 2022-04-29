use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Program {
    Let(Variable, Expr),
    Expr(Expr),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Expr {
    Variable(Variable),
    Assign(Variable, Box<Expr>),
    Literal(Literal),
    Function(Function),
    Call(String, Vec<Expr>),
    Operator(Operator),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub global: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Function {
    pub args: Vec<FunctionArg>,
    pub rtn: String,
    pub body: Vec<Program>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionArg {
    pub arg: Variable,
    pub annotation: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Operator {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Tensor(Vec<Literal>),
}
