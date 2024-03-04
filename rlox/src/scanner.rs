use std::string;
use std::collections::HashMap;


#[derive(PartialEq, Debug, Clone)]
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
    IDENTIFIER(String), STRING(String), NUMBER(f64), 

    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
}

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
/**
 * Returns None if end of file and no closing "
 */
fn get_string_literal_token(iter: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Option<Token> {
    let mut string_lit = String::new();

    while let Some(next) = iter.next() {
        // TODO: line increments
        if next == '"' { // done with string literal
            return Some(Token::STRING(string_lit))
        }
        string_lit.push(next);
    }

    return None
}

fn is_digit(c: char) -> bool {
    match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
        _ => false
    }
}

/**
 * Returns biggest numeric string it can get from the iter
 */
fn get_numeric_string(iter: &mut std::iter::Peekable<std::str::Chars<'_>>) -> String {
    let mut numeric_literal = String::new();

    while let Some(next) = iter.peek() {
        let next_char = *next;
        if is_digit(next_char){
            numeric_literal.push(next_char);
            iter.next();
        }
        else {
            break;
        }
    }

    numeric_literal
}

fn get_numeric_literal(iter: &mut std::iter::Peekable<std::str::Chars<'_>>, first_char: char) -> Token {
    let mut numeric_literal = String::new();
    numeric_literal.push(first_char);

    numeric_literal = numeric_literal + &get_numeric_string(iter);

    // add fractional part
    let final_numeric_lit = match iter.peek() {
        Some(next) => {
            if *next == '.' { 
                // TODO: needs one more peek ahead to check if actually digit after .
                // requires external crate lol, for later
                numeric_literal.push(iter.next().expect("We just peeked it"));

                numeric_literal + &get_numeric_string(iter)
            }
            else {
                numeric_literal
            }
        }
        None => numeric_literal
    };

    Token::NUMBER(final_numeric_lit.parse().expect("This should be a valid number"))
}


fn get_identifers_or_keywords(iter: &mut std::iter::Peekable<std::str::Chars<'_>>, first_char: char) -> Token {
    let mut identifier = String::new();
    identifier.push(first_char);

    while let Some(next) = iter.peek(){
        if next.is_ascii_alphanumeric(){
            identifier.push(iter.next().expect("We just peeked it"));
        }
        else {
            break
        }
    }

    // TODO: make this not run every fucking time
    let mut keyword_map = HashMap::new();
    keyword_map.insert("and",    Token::AND);
    keyword_map.insert("class",  Token::CLASS);
    keyword_map.insert("else",   Token::ELSE);
    keyword_map.insert("false",  Token::FALSE);
    keyword_map.insert("for",    Token::FOR);
    keyword_map.insert("fun",    Token::FUN);
    keyword_map.insert("if",     Token::IF);
    keyword_map.insert("nil",    Token::NIL);
    keyword_map.insert("or",     Token::OR);
    keyword_map.insert("print",  Token::PRINT);
    keyword_map.insert("return", Token::RETURN);
    keyword_map.insert("super",  Token::SUPER);
    keyword_map.insert("this",   Token::THIS);
    keyword_map.insert("true",   Token::TRUE);
    keyword_map.insert("var",    Token::VAR);
    keyword_map.insert("while",  Token::WHILE);

    keyword_map.get(identifier.as_str()).cloned().unwrap_or(Token::IDENTIFIER(identifier))
}

/**
 * Returns None at end of file
 */
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
                    // ignore rest of line
                    while let Some(next) = iter.peek() {
                        if *next == '\n' {
                            break;
                        }
                        iter.next();
                    }
                    // lol fuck
                    continue 'main_loop;
                }

                // else, not double //
                Token::SLASH
            }

            ' ' | '\r' | '\t' => continue 'main_loop,
            '\n' => continue 'main_loop, //TODO: line increment

            '"' => get_string_literal_token(iter).expect("unclosed string literal"),
            other => {
                if is_digit(other) {
                    get_numeric_literal(iter, other)
                }
                else if other.is_ascii_alphanumeric() {
                    get_identifers_or_keywords(iter, other)
                }
                else {
                    todo!()
                }
            }
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
        assert_eq!(scanner::scan_tokens(& String::from("     ")), vec![Token::EOF]);
        assert_eq!(scanner::scan_tokens(& String::from("!=")), vec![Token::BANG_EQUAL, Token::EOF]);
        assert_eq!(scanner::scan_tokens(& String::from("!!=")), vec![Token::BANG, Token::BANG_EQUAL, Token::EOF]);
        assert_eq!(scanner::scan_tokens(& String::from("/")), vec![Token::SLASH, Token::EOF]);

        assert_eq!(scanner::scan_tokens(& String::from("//")), vec![Token::EOF]);
        assert_eq!(scanner::scan_tokens(& String::from("//+")), vec![Token::EOF]);
        assert_eq!(scanner::scan_tokens(& String::from("//\n")), vec![Token::EOF]);
        assert_eq!(scanner::scan_tokens(& String::from("//\n+")), vec![Token::PLUS, Token::EOF]);
        assert_eq!(scanner::scan_tokens(& String::from("//asdsad asd \n+")), vec![Token::PLUS, Token::EOF]);

        assert_eq!(scanner::scan_tokens(& String::from(r#""blah""#)), vec![Token::STRING(String::from("blah")), Token::EOF]);

        assert_eq!(scanner::scan_tokens(& String::from("123")), vec![Token::NUMBER("123".parse().unwrap()), Token::EOF]);
        assert_eq!(scanner::scan_tokens(& String::from("1.1")), vec![Token::NUMBER("1.1".parse().unwrap()), Token::EOF]);
        assert_eq!(scanner::scan_tokens(& String::from("1.1 2.2")), vec![Token::NUMBER("1.1".parse().unwrap()), 
        Token::NUMBER("2.2".parse().unwrap()), Token::EOF]);

        assert_eq!(scanner::scan_tokens(& String::from("fuck")), vec![Token::IDENTIFIER(String::from("fuck")), Token::EOF]);
        assert_eq!(scanner::scan_tokens(& String::from("and")), vec![Token::AND, Token::EOF]);
        assert_eq!(scanner::scan_tokens(& String::from("and fuck")), vec![Token::AND, 
        Token::IDENTIFIER(String::from("fuck")), Token::EOF]);
    }
}
