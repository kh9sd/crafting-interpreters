use crate::parser::Expr;
use crate::parser::Stmt;
pub use crate::scanner::Token;

#[derive(PartialEq, Debug, Clone)]
pub enum ValueType {
    Number(f64),
    Boolean(bool),
    String(String),
    Nil,
}

pub fn stmt_eval(statement: &Stmt) {
    match statement {
        Stmt::Expression(expr) => {
            let _ = evaluate(expr);
            // do nothing???
        },
        Stmt::Print(expr) => {
            println!("{:?}", stringify_valuetype(&evaluate(expr)));
        }
    }
}

fn evaluate(expression_tree: &Expr) -> ValueType{
    match expression_tree {
        Expr::Binary(left, op, right) => binary_funct(op, left, right),
        Expr::Unary(op, expr) => unary_funct(op, expr),
        Expr::Grouping(expr) => evaluate(expr),

        // //terminals
        Expr::String(str) => ValueType::String(str.clone()),
        Expr::Number(x) => ValueType::Number(*x),
        Expr::Boolean(bool) => ValueType::Boolean(*bool),
        Expr::Nil => ValueType::Nil
    }
}

pub fn stringify_valuetype(val: &ValueType) -> String{
    match val {
        ValueType::Boolean(b) => b.to_string(),
        ValueType::Nil => String::from("Nil"),
        ValueType::Number(x) => x.to_string(),
        ValueType::String(s) => s.clone()
    }
}

fn binary_funct(operation: &Token, left_expr: &Expr, right_expr: &Expr) -> ValueType {
    match operation {
        Token::STAR => {
            let left_res = evaluate(left_expr);
            let right_res = evaluate(right_expr);
    
            match (left_res, right_res){
                (ValueType::Number(x), ValueType::Number(y)) => ValueType::Number(x*y),
                (a, b) => panic!("Bad arguments {:?} and {:?} to * operator", a, b)
            }
        },
        Token::SLASH => {
            let left_res = evaluate(left_expr);
            let right_res = evaluate(right_expr);
    
            match (left_res, right_res){
                (ValueType::Number(x), ValueType::Number(y)) => ValueType::Number(x/y),
                (a, b) => panic!("Bad arguments {:?} and {:?} to / operator", a, b)
            }
        },
        Token::MINUS => {
            let left_res = evaluate(left_expr);
            let right_res = evaluate(right_expr);
    
            match (left_res, right_res){
                (ValueType::Number(x), ValueType::Number(y)) => ValueType::Number(x-y),
                (a, b) => panic!("Bad arguments {:?} and {:?} to - operator", a, b)
            }
        },
        Token::PLUS => {
            let left_res = evaluate(left_expr);
            let right_res = evaluate(right_expr);
    
            match (left_res, right_res){
                (ValueType::Number(x), ValueType::Number(y)) => ValueType::Number(x+y),
                (ValueType::String(x), ValueType::String(y)) => ValueType::String(x+&y),
                (a, b) => panic!("Bad arguments {:?} and {:?} to + operator", a, b)
            }
        },

        Token::GREATER => {
            let left_res = evaluate(left_expr);
            let right_res = evaluate(right_expr);

            match (left_res, right_res){
                (ValueType::Number(x), ValueType::Number(y)) => ValueType::Boolean(x > y),
                (a, b) => panic!("Bad arguments {:?} and {:?} to > operator", a, b)
            }
        },
        Token::GREATER_EQUAL => {
            let left_res = evaluate(left_expr);
            let right_res = evaluate(right_expr);

            match (left_res, right_res){
                (ValueType::Number(x), ValueType::Number(y)) => ValueType::Boolean(x >= y),
                (a, b) => panic!("Bad arguments {:?} and {:?} to >= operator", a, b)
            }
        },
        Token::LESS => {
            let left_res = evaluate(left_expr);
            let right_res = evaluate(right_expr);

            match (left_res, right_res){
                (ValueType::Number(x), ValueType::Number(y)) => ValueType::Boolean(x < y),
                (a, b) => panic!("Bad arguments {:?} and {:?} to < operator", a, b)
            }
        },
        Token::LESS_EQUAL => {
            let left_res = evaluate(left_expr);
            let right_res = evaluate(right_expr);

            match (left_res, right_res){
                (ValueType::Number(x), ValueType::Number(y)) => ValueType::Boolean(x <= y),
                (a, b) => panic!("Bad arguments {:?} and {:?} to <= operator", a, b)
            }
        },
        Token::BANG_EQUAL =>
            ValueType::Boolean(!is_equal(&evaluate(left_expr), &evaluate(right_expr))),
        Token::EQUAL_EQUAL =>
            ValueType::Boolean(is_equal(&evaluate(left_expr), &evaluate(right_expr))),
    
        other => panic!("Bad binary operator: {:?}", other)
    }
}

fn is_equal(left: &ValueType, right: &ValueType) -> bool{
    match (left, right){
        (ValueType::Nil, ValueType::Nil) => true,
        (ValueType::Number(x), ValueType::Number(y)) => x == y, // TODO: == on double, sus
        (ValueType::Boolean(x), ValueType::Boolean(y)) => x == y,
        (ValueType::String(x), ValueType::String(y)) => x == y,
        (_, _) => false
    }
}

fn unary_funct(operation: &Token, expression: &Expr) -> ValueType {
    match operation {
        Token::MINUS => {
            let sub_expr_res = evaluate(expression);
            match sub_expr_res {
                ValueType::Number(x) => ValueType::Number(-x),

                _ => panic!("Runtime error, expected number to unary - operator")
            }
        },
        Token::BANG => {
            let sub_expr_res = evaluate(expression);
            match sub_expr_res {
                ValueType::Boolean(b) => ValueType::Boolean(!b),
                ValueType::Nil => ValueType::Boolean(true),

                _ => panic!("Runtime error, expected truthy value to unary ! operator")
            }
        }
        other => panic!("Not supported unary expression: {:?}", other)
    }
}
