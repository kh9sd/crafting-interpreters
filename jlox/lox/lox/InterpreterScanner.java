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
        
    }
}
