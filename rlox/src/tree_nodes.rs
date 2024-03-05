
pub use crate::scanner::Token;

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    //nonterminals
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Unary(Token, Box<Expr>),
    //terminals
    String(String),
    Number(f64),
    Boolean(bool),
    Nil
}

pub enum ValueType {
    Number(f64),
    Boolean(bool),
    Nil,
}
