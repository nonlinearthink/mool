use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum IConstant {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Expr {
    Literal(String),
    Constant(IConstant),
    Ref(String),
    Assign(String, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}
