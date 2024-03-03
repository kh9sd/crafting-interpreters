

#[derive(PartialEq, Debug)]
pub enum Token{
    // Single-character tokens.
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // One or two character tokens.
    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,

    // Literals.
    IDENTIFIER, STRING, NUMBER,

    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
}

// fn scan_tokens(source_code: String) -> Vec<Token>{
//     let mut result = Vec::new();

//     let mut start: u32 = 0;
//     let mut current: u32 = 0;
//     let mut line: u32 = 1;

    
    
//     result.push(Token::EOF);
//     return result;
// }


fn scan_single_token(iter: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Option<Token> {
    while let Some(c) = iter.next() {
        let current: char = c;

        let next_token = match current {
            // ez ones
            '(' => Token::LEFT_PAREN,
            ')' => Token::RIGHT_PAREN,
            '{' => Token::LEFT_BRACE,
            '}' => Token::RIGHT_BRACE,
            ',' => Token::COMMA,
            '.' => Token::DOT,
            '-' => Token::MINUS,
            '+' => Token::PLUS,
            ';' => Token::SEMICOLON,
            '*' => Token::STAR,

            '!' => {
                let next: char = *iter.peek().unwrap_or(&'\0');

                if next == '='{
                    iter.next();
                    Token::BANG_EQUAL
                }
                else {
                    Token::BANG
                }
            },

            '=' => {
                let next: char = *iter.peek().unwrap_or(&'\0');

                if (next == '='){
                    iter.next();
                    Token::EQUAL_EQUAL
                }
                else {
                    Token::EQUAL
                }
            },

            '<' => {
                let next: char = *iter.peek().unwrap_or(&'\0');

                if next == '='{
                    iter.next();
                    Token::LESS_EQUAL
                }
                else {
                    Token::LESS
                }
            },

            '>' => {
                let next: char = *iter.peek().unwrap_or(&'\0');

                if next == '='{
                    iter.next();
                    Token::GREATER_EQUAL
                }
                else {
                    Token::GREATER
                }
            },

            _ => todo!()
        };

        return Some(next_token)
    }

    return None;
}


fn scan_tokens(source_code: &String) -> Vec<Token> {
    let mut result = Vec::new();

    let mut iter: std::iter::Peekable<std::str::Chars<'_>> = source_code.chars().peekable();
    
    while let Some(c) = scan_single_token(&mut iter) {
        result.push(c);
    }

    result.push(Token::EOF);
    return result;
}



#[cfg(test)]
mod tests {
    use crate::scanner::Token;
    
    use crate::scanner;

    #[test]
    fn scan_tokens_tests() {
        assert_eq!(scanner::scan_tokens(& String::from("+")), vec![Token::PLUS, Token::EOF]);
        assert_eq!(scanner::scan_tokens(& String::from("")), vec![Token::EOF]);
        assert_eq!(scanner::scan_tokens(& String::from("!=")), vec![Token::BANG_EQUAL, Token::EOF]);
        assert_eq!(scanner::scan_tokens(& String::from("!!=")), vec![Token::BANG, Token::BANG_EQUAL, Token::EOF]);
    }
}
