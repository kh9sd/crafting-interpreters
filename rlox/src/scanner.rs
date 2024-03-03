

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

fn look_ahead_1_char(iter: &mut std::iter::Peekable<std::str::Chars<'_>>, char_to_match: char, if_match: Token, else_match: Token) -> Token {
    match iter.peek() {
        Some(char) => {
            if (*char == char_to_match){
                // consume this char
                iter.next();
                if_match
            }
            else {
                else_match
            }
        }
        None => else_match
    }
}

fn scan_single_token(iter: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Option<Token> {
    'main_loop: while let Some(current) = iter.next() {
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

            '!' =>
                look_ahead_1_char(iter, '=', Token::BANG_EQUAL, Token::BANG),

            '=' =>
                look_ahead_1_char(iter, '=', Token::EQUAL_EQUAL, Token::EQUAL),

            '<' =>
                look_ahead_1_char(iter, '=', Token::LESS_EQUAL, Token::LESS),

            '>' =>
                look_ahead_1_char(iter, '=', Token::GREATER_EQUAL, Token::GREATER),

            //division or comment
            '/' => {
                let next_char = *iter.peek().unwrap_or(&'\0');
                if next_char == '/' {
                    while let Some(next) = iter.peek() {
                        if *next == '\n' {
                            break;
                        }
                        iter.next();
                    }
                    // lol fuck
                    continue 'main_loop;
                }

                
                Token::SLASH
            }

            ' ' | '\r' | '\t' => continue 'main_loop,
            '\n' => continue 'main_loop, //TODO: line increment

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
        assert_eq!(scanner::scan_tokens(& String::from("/")), vec![Token::SLASH, Token::EOF]);

        assert_eq!(scanner::scan_tokens(& String::from("//")), vec![Token::EOF]);
        assert_eq!(scanner::scan_tokens(& String::from("//+")), vec![Token::EOF]);
        assert_eq!(scanner::scan_tokens(& String::from("//\n")), vec![Token::EOF]);
        assert_eq!(scanner::scan_tokens(& String::from("//\n+")), vec![Token::PLUS, Token::EOF]);
    }
}
