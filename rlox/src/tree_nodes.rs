
mod scanner;
use crate::scanner::Token;

pub enum Expr {
    //nonterminals
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Unary(Token, Box<Expr>),
    //terminals
    String(String),
    Number(f64),
    Boolean(bool),
}

