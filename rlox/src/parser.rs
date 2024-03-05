use crate::tree_nodes::Expr;
pub use crate::scanner::Token;
use std::iter::Peekable;


pub fn parse(token_list: Vec<Token>) -> Expr {
    let mut iter = token_list.iter().peekable();
    
    primary(&mut iter)
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
        assert_eq!(parse(vec![Token::TRUE, Token::EOF]), Expr::Boolean(true));
    }
}
