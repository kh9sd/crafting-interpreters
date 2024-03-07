use std::iter::Peekable;

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

#[derive(PartialEq, Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
}

// fn program


/**
 * program        → statement* EOF ;

    statement      → exprStmt
                | printStmt ;

    exprStmt       → expression ";" ;
    printStmt      → "print" expression ";" ;
 */
fn program(iter: &mut Peekable<std::slice::Iter<'_, Token>>) -> Vec<Stmt> {
    let mut result = Vec::new();

    while let Some(_) = iter.peek() {
        result.push(statement(iter));
    };

    result
}

fn statement(iter: &mut Peekable<std::slice::Iter<'_, Token>>) -> Stmt {
    match iter.peek().expect("Iterator should not be exhausted") {
        Token::PRINT => {
            // consume print token
            iter.next();
            printStmt(iter)
        }
        _ => {
            exprStmt(iter)
        }
    }
}

fn printStmt(iter: &mut Peekable<std::slice::Iter<'_, Token>>) -> Stmt {
    let expr = expression(iter);

    match iter.next().expect("Iterator should not be empty"){
        Token::SEMICOLON => Stmt::Print(expr),
        other => panic!("Bad token after expression, {:?}", other)
    }
}

fn exprStmt(iter: &mut Peekable<std::slice::Iter<'_, Token>>) -> Stmt {
    let expr = expression(iter);

    match iter.next().expect("Iterator should not be empty"){
        Token::SEMICOLON => Stmt::Expression(expr),
        other => panic!("Bad token after expression, {:?}", other)
    }
}




/**
 * Expression grammar impl
 */
fn expression(iter: &mut Peekable<std::slice::Iter<'_, Token>>) -> Expr {
    equality(iter)
}
/**
 * expression     → equality ;
    equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    term           → factor ( ( "-" | "+" ) factor )* ;
    factor         → unary ( ( "/" | "*" ) unary )* ;
    unary          → ( "!" | "-" ) unary
                | primary ;
    primary        → NUMBER | STRING | "true" | "false" | "nil"
                | "(" expression ")" ;
 */

fn equality(iter: &mut Peekable<std::slice::Iter<'_, Token>>) -> Expr {
    let mut result = comparison(iter);

    loop
    {
        match iter.peek().expect("Iterator should not be exhausted") {
            Token::BANG_EQUAL | Token::EQUAL_EQUAL =>
            {
                result = Expr::Binary(Box::new(result), 
                    iter.next().expect("We just peeked").clone(), 
                    Box::new(comparison(iter)))
            },
            _ => break
        }
    }
    result
}

// >, <, >= and <=
fn comparison(iter: &mut Peekable<std::slice::Iter<'_, Token>>) -> Expr {
    let mut result = term(iter);

    loop
    {
        match iter.peek().expect("Iterator should not be exhausted") {
            Token::GREATER | Token::GREATER_EQUAL |
            Token::LESS | Token::LESS_EQUAL =>
            {
                result = Expr::Binary(Box::new(result), 
                    iter.next().expect("We just peeked").clone(), 
                    Box::new(term(iter)))
            },
            _ => break
        }
    }
    result
}

// + and -
fn term(iter: &mut Peekable<std::slice::Iter<'_, Token>>) -> Expr {
    let mut result = factor(iter);

    loop
    {
        match iter.peek().expect("Iterator should not be exhausted") {
            Token::PLUS | Token::MINUS =>
            {
                result = Expr::Binary(Box::new(result), 
                    iter.next().expect("We just peeked").clone(), 
                    Box::new(factor(iter)))
            },
            _ => break
        }
    }
    result
}


// * and /
fn factor(iter: &mut Peekable<std::slice::Iter<'_, Token>>) -> Expr {
    let mut result = unary(iter);

    loop
    {
        match iter.peek().expect("Iterator should not be exhausted") {
            Token::STAR | Token::SLASH =>
            {
                result = Expr::Binary(Box::new(result), 
                    iter.next().expect("We just peeked").clone(), 
                    Box::new(unary(iter)))
            },
            _ => break
        }
    }
    result
}

// - and !
fn unary(iter: &mut Peekable<std::slice::Iter<'_, Token>>) -> Expr {
    let next_token = iter.peek().expect("Iterator should not be exhausted");

    match next_token {
        Token::MINUS | Token::BANG => 
        {
            Expr::Unary(iter.next().expect("We just peeked").clone(), Box::new(unary(iter)))
        }
        _ => primary(iter)
    }
}

fn primary(iter: &mut Peekable<std::slice::Iter<'_, Token>>) -> Expr {
    let next_token = iter.next().expect("Iterator should not be exhausted").clone();

    match next_token {
        Token::NUMBER(x) => Expr::Number(x),
        Token::STRING(str) => Expr::String(str),
        Token::TRUE => Expr::Boolean(true),
        Token::FALSE => Expr::Boolean(false),
        Token::NIL => Expr::Nil,
        Token::LEFT_PAREN => {
            let expr = expression(iter);
            match iter.next() {
                Some(x) => {
                    if let Token::RIGHT_PAREN = x {
                        // good, just consume and move on
                    }
                    else{
                        panic!("Expected closing parenthesis")
                    }
                }
                None => panic!("Expected closing parenthesis")
            };

            Expr::Grouping(Box::new(expr))
        },
        _ => todo!("implement parenthesis case")
    }
}


fn parse(token_list: &Vec<Token>) -> Expr {
    let mut iter = token_list.iter().peekable();
    
    expression(&mut iter)
}
#[cfg(test)]
mod tests {
    use crate::{parser::parse, scanner::Token};
    use crate::parser::Expr;
    #[test]
    fn parse_tests() {
        // terminals
        assert_eq!(parse(&vec![Token::TRUE, Token::EOF]), Expr::Boolean(true));
        assert_eq!(parse(&vec![Token::FALSE, Token::EOF]), Expr::Boolean(false));
        assert_eq!(parse(&vec![Token::NIL, Token::EOF]), Expr::Nil);
        assert_eq!(parse(&vec![Token::STRING(String::from("blah")), 
            Token::EOF]), Expr::String(String::from("blah")));
        assert_eq!(parse(&vec![Token::NUMBER(0.0), Token::EOF]), Expr::Number(0.0));

        // unary 
        assert_eq!(parse(&vec![Token::BANG, Token::TRUE, Token::EOF]), 
            Expr::Unary(Token::BANG, 
                Box::new(Expr::Boolean(true))));

        assert_eq!(parse(&vec![Token::MINUS, Token::NUMBER(0.0), Token::EOF]), 
            Expr::Unary(Token::MINUS, 
                Box::new(Expr::Number(0.0))));
        
        // factor
        assert_eq!(parse(&vec![Token::NUMBER(0.0), Token::STAR, 
                            Token::NUMBER(0.0), Token::SLASH,
                            Token::NUMBER(0.0),
                            Token::EOF]), 
            Expr::Binary(
                Box::new(
                    Expr::Binary(Box::new(Expr::Number(0.0)),
                                Token::STAR,
                                Box::new(Expr::Number(0.0)))), 
                Token::SLASH,
                Box::new(Expr::Number(0.0))));


        assert_eq!(parse(&vec![Token::MINUS, Token::NUMBER(0.0), Token::STAR, 
                Token::MINUS, Token::NUMBER(0.0),
                Token::EOF]), 
        Expr::Binary(
            Box::new(
                Expr::Unary(Token::MINUS,
                    Box::new(Expr::Number(0.0)))), 
            Token::STAR,
            Box::new(
                Expr::Unary(Token::MINUS,
                    Box::new(Expr::Number(0.0))))));
        
        // term
        assert_eq!(parse(&vec![Token::NUMBER(0.0), Token::MINUS, 
                            Token::NUMBER(0.0), Token::SLASH,
                            Token::NUMBER(0.0),
                            Token::EOF]), 
            Expr::Binary(
                Box::new(Expr::Number(0.0)),
                Token::MINUS,
                Box::new(
                    Expr::Binary(Box::new(Expr::Number(0.0)),
                                Token::SLASH,
                                Box::new(Expr::Number(0.0))))));

        
        assert_eq!(parse(&vec![Token::NUMBER(0.0), Token::STAR, 
                            Token::NUMBER(0.0), Token::PLUS,
                            Token::NUMBER(0.0),
                            Token::EOF]), 
            Expr::Binary(
                Box::new(
                    Expr::Binary(Box::new(Expr::Number(0.0)),
                                Token::STAR,
                                Box::new(Expr::Number(0.0)))),
                Token::PLUS,
                Box::new(Expr::Number(0.0))));

        //comparison
        assert_eq!(parse(&vec![Token::NUMBER(0.0), Token::GREATER, 
                            Token::NUMBER(0.0), Token::PLUS,
                            Token::NUMBER(0.0),
                            Token::EOF]), 
            Expr::Binary(
                Box::new(Expr::Number(0.0)),
                Token::GREATER,
                Box::new(
                    Expr::Binary(Box::new(Expr::Number(0.0)),
                                Token::PLUS,
                                Box::new(Expr::Number(0.0))))));

        
        assert_eq!(parse(&vec![Token::NUMBER(0.0), Token::MINUS, 
                            Token::NUMBER(0.0), Token::LESS_EQUAL,
                            Token::NUMBER(0.0),
                            Token::EOF]), 
            Expr::Binary(
                Box::new(
                    Expr::Binary(Box::new(Expr::Number(0.0)),
                                Token::MINUS,
                                Box::new(Expr::Number(0.0)))),
                Token::LESS_EQUAL,
                Box::new(Expr::Number(0.0))));
        

        //equality
        assert_eq!(parse(&vec![Token::NUMBER(0.0), Token::BANG_EQUAL, 
                            Token::NUMBER(0.0), Token::LESS,
                            Token::NUMBER(0.0),
                            Token::EOF]), 
            Expr::Binary(
                Box::new(Expr::Number(0.0)),
                Token::BANG_EQUAL,
                Box::new(
                    Expr::Binary(Box::new(Expr::Number(0.0)),
                                Token::LESS,
                                Box::new(Expr::Number(0.0))))));

        
        assert_eq!(parse(&vec![Token::NUMBER(0.0), Token::GREATER_EQUAL, 
                            Token::NUMBER(0.0), Token::EQUAL_EQUAL,
                            Token::NUMBER(0.0),
                            Token::EOF]), 
            Expr::Binary(
                Box::new(
                    Expr::Binary(Box::new(Expr::Number(0.0)),
                                Token::GREATER_EQUAL,
                                Box::new(Expr::Number(0.0)))),
                Token::EQUAL_EQUAL,
                Box::new(Expr::Number(0.0))));
        
        // parenthesis
        assert_eq!(parse(&vec![Token::NUMBER(0.0), Token::STAR, 
                            Token::LEFT_PAREN,
                            Token::NUMBER(0.0), Token::PLUS,
                            Token::NUMBER(0.0),
                            Token::RIGHT_PAREN,
                            Token::EOF]), 

            Expr::Binary(Box::new(Expr::Number(0.0)),
                        Token::STAR,
                        Box::new(Expr::Grouping(
                            Box::new(Expr::Binary(
                                Box::new(Expr::Number(0.0)),
                                Token::PLUS,
                                Box::new(Expr::Number(0.0))))))));
    }
}
