use crate::tree_nodes::Expr;
pub use crate::scanner::Token;
use std::iter::Peekable;


pub fn parse(token_list: Vec<Token>) -> Expr {
    let mut iter = token_list.iter().peekable();
    
    unary(&mut iter)
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

fn equality(iter: Peekable<std::slice::Iter<'_, Token>>) -> Expr {


    Expr::Number(0.0)
}

fn comparison(iter: Peekable<std::slice::Iter<'_, Token>>) -> Expr {
    Expr::Number(0.0)
}


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
        _ => todo!("implement parenthesis case")
    }
}


#[cfg(test)]
mod tests {
    use crate::{parser::parse, scanner::Token};
    use crate::parser::Expr;
    #[test]
    fn parse_tests() {
        // terminals
        assert_eq!(parse(vec![Token::TRUE, Token::EOF]), Expr::Boolean(true));
        assert_eq!(parse(vec![Token::FALSE, Token::EOF]), Expr::Boolean(false));
        assert_eq!(parse(vec![Token::NIL, Token::EOF]), Expr::Nil);
        assert_eq!(parse(vec![Token::STRING(String::from("blah")), 
            Token::EOF]), Expr::String(String::from("blah")));
        assert_eq!(parse(vec![Token::NUMBER(0.0), Token::EOF]), Expr::Number(0.0));

        // unary 
        assert_eq!(parse(vec![Token::BANG, Token::TRUE, Token::EOF]), 
            Expr::Unary(Token::BANG, 
                Box::new(Expr::Boolean(true))));

        assert_eq!(parse(vec![Token::MINUS, Token::NUMBER(0.0), Token::EOF]), 
            Expr::Unary(Token::MINUS, 
                Box::new(Expr::Number(0.0))));

    }
}
