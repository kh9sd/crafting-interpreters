package lox;

import java.util.ArrayList;
import java.util.List;

class InterpreterScanner {
    private final String source_code;
    private int start = 0;
    private int current = 0;
    private int line = 1;

    private final List<Token> tokens = new ArrayList<>();

    InterpreterScanner(String source_code) {
        this.source_code = source_code;
    }

    List<Token> scanTokens(){
        while (!this.isAtEnd()) {
            // We are at the beginning of the next lexeme.
            this.start = this.current;
            this.scanToken();
        }

        this.tokens.add(new Token(TokenType.EOF, "", null, this.line));
        return this.tokens;
    }

    private boolean isAtEnd(){
        return current >= source_code.length();
    }

    private void scanToken(){
        char c = this.advance();
        switch (c) {
            // ez ones
            case '(':
                this.addToken(TokenType.LEFT_PAREN);
                break;
            case ')':
                this.addToken(TokenType.RIGHT_PAREN);
                break;
            case '{':
                this.addToken(TokenType.LEFT_BRACE);
                break;
            case '}':
                this.addToken(TokenType.RIGHT_BRACE);
                break;
            case ',':
                this.addToken(TokenType.COMMA);
                break;
            case '.':
                this.addToken(TokenType.DOT);
                break;
            case '-':
                this.addToken(TokenType.MINUS);
                break;
            case '+':
                this.addToken(TokenType.PLUS);
                break;
            case ';':
                this.addToken(TokenType.SEMICOLON);
                break;
            case '*':
                this.addToken(TokenType.STAR);
                break;

            // harder ones
            case '!':
                if (this.peek() == '='){
                    this.advance();
                    this.addToken(TokenType.BANG_EQUAL);
                }
                else {
                    this.addToken(TokenType.BANG);
                }
                break;

            case '=':
                if (this.peek() == '='){
                    this.advance();
                    this.addToken(TokenType.EQUAL_EQUAL);
                }
                else {
                    this.addToken(TokenType.EQUAL);
                }
                break;

            case '<':
                if (this.peek() == '='){
                    this.advance();
                    this.addToken(TokenType.LESS_EQUAL);
                }
                else {
                    this.addToken(TokenType.LESS);
                }
                break;

            case '>':
                if (this.peek() == '='){
                    this.advance();
                    this.addToken(TokenType.GREATER_EQUAL);
                }
                else {
                    this.addToken(TokenType.GREATER);
                }
                break;

            // this final really hard case for comments/division
            case '/':
                if (this.peek() == '/'){
                    // advance till end of line or file
                    while (this.peek() != '\n' && this.peek() != '\0'){
                        this.advance();
                    }
                }
                else {
                    this.addToken(TokenType.SLASH);
                }
                break;


            case ' ':
            case '\r':
            case '\t':
                // Ignore whitespace.
                break;

            case '\n':
                this.line++;
                break;

            default:
                Lox.error(this.line, "Unexpected character.");
                break;
        }
    }

    private void addToken(TokenType tt){
        String text = source_code.substring(start, current);
        this.tokens.add(new Token(tt, text, null, line));
    }

    private char peek(){
        if (this.isAtEnd()){
            return '\0';
        }
        return this.source_code.charAt(this.current);
    }

    private char advance(){
        char result = this.peek();
        this.current++;
        return result;
    }
}
