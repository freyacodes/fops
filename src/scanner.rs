use crate::scanner::TokenType::*;
use strum::VariantArray;

pub struct Scanner<'a> {
    source: &'a str,
    current_token_start: usize,
    current_token_end: usize,
    line: usize,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Token<'a> {
    pub string: &'a str,
    pub token_type: TokenType,
    pub line: usize,
}

pub const PLACEHOLDER_TOKEN: Token = Token {
    string: "",
    token_type: EOF,
    line: 0
};

#[derive(Debug, PartialEq, Copy, Clone, VariantArray)]
#[rustfmt::skip]
pub enum TokenType {
    // Single character tokens
    TokenLeftParen, TokenRightParen,
    TokenLeftBrace, TokenRightBrace,
    TokenComma, TokenDot, TokenMinus, TokenPlus,
    TokenSemicolon, TokenSlash, TokenAsterisk,

    // One or two character tokens
    TokenBang, TokenBangEqual,
    TokenEqual, TokenEqualEqual,
    TokenLess, TokenLessEqual,
    TokenGreater, TokenGreaterEqual,
    TokenAmpAmp, TokenPipePipe,

    // Literals
    TokenIdentifier, TokenString, TokenNumber,

    // Keywords
    TokenElse, TokenFalse, TokenFun, TokenLet, TokenNil,
    TokenIf, TokenRepeat, TokenReturn, TokenTrue, TokenWhile,
    
    // Non-tokens
    EOF, ScannerError
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        Self {
            source,
            current_token_start: 0,
            current_token_end: 0,
            line: 1,
        }
    }

    pub fn next(&mut self) -> Token<'a> {
        // Skip whitespace
        loop {
            if self.is_at_end() {
                return Token { token_type: EOF, string: "", line: self.line };
            }
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == Some('/') {
                        self.skip_until_line_end();
                        continue;
                    }
                    break;
                }
                _ => {
                    break;
                }
            }
        }

        self.current_token_start = self.current_token_end;

        let c = self.peek();
        self.advance();

        if c.is_alphabetic() || c == '_' {
            return self.identifier()
        } else if c.is_ascii_digit() {
            return self.number_literal()
        }

        match c {
            '(' => self.make_token(TokenLeftParen),
            ')' => self.make_token(TokenRightParen),
            '{' => self.make_token(TokenLeftBrace),
            '}' => self.make_token(TokenRightBrace),
            ';' => self.make_token(TokenSemicolon),
            ',' => self.make_token(TokenComma),
            '.' => self.make_token(TokenDot),
            '-' => self.make_token(TokenMinus),
            '+' => self.make_token(TokenPlus),
            '/' => self.make_token(TokenSlash),
            '*' => self.make_token(TokenAsterisk),
            '!' => {
                if self.match_next('=') {
                    self.make_token(TokenBangEqual)
                } else {
                    self.make_token(TokenBang)
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.make_token(TokenEqualEqual)
                } else {
                    self.make_token(TokenEqual)
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.make_token(TokenLessEqual)
                } else {
                    self.make_token(TokenLess)
                }
            }
            '>' => {
                if self.match_next('=') {
                    self.make_token(TokenGreaterEqual)
                } else {
                    self.make_token(TokenGreater)
                }
            }
            '&' => {
                if self.match_next('&') {
                    self.make_token(TokenAmpAmp)
                } else {
                    self.make_error("Unexpected lone &")
                }
            }
            '|' => {
                if self.match_next('|') {
                    self.make_token(TokenPipePipe)
                } else {
                    self.make_error("Unexpected |")
                }
            }
            '"' => self.string_literal(),
            _ => self.make_error("Unexpected character")
        }
    }

    fn peek(&self) -> char {
        self.source.as_bytes()[self.current_token_end] as char
    }

    fn peek_next(&self) -> Option<char> {
        let i = self.current_token_end + 1;
        if self.source.len() <= i {
            None
        } else {
            Some(self.source.as_bytes()[i] as char)
        }
    }

    fn skip_until_line_end(&mut self) {
        loop {
            self.advance();
            if self.is_at_end() || self.peek() == '\n' {
                break;
            }
        }
    }

    fn advance(&mut self) {
        self.current_token_end += 1;
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.as_bytes()[self.current_token_end] as char != expected {
            return false;
        }
        self.advance();
        true
    }

    fn is_at_end(&self) -> bool {
        self.source.len() == self.current_token_end
    }

    fn string_literal(&mut self) -> Token<'a> {
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.make_error("Unterminated string literal");
        }

        self.advance();
        self.make_token(TokenString)
    }

    fn identifier(&mut self) -> Token<'a> {
        loop {
            if self.is_at_end() { break; }
            let c = self.peek();
            if !c.is_ascii_alphanumeric() && c != '_' {
                break;
            }
            self.advance();
        }

        self.make_token(Self::identifier_type(self.get_current_string()))
    }

    fn number_literal(&mut self) -> Token<'a> {
        while !self.is_at_end() && self.peek().is_ascii_digit() {
            self.advance();
        }

        if !self.is_at_end() && self.peek() == '.' {
            self.advance();
            while !self.is_at_end() && self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.make_token(TokenNumber)
    }

    fn make_token(&self, token_type: TokenType) -> Token<'a> {
        Token {
            string: self.get_current_string(),
            token_type,
            line: self.line,
        }
    }
    
    fn make_error(&self, message: &'static str) -> Token<'static> {
        Token {
            string: message,
            token_type: ScannerError,
            line: self.line,
        }
    }

    fn get_current_string(&self) -> &'a str {
        &self.source[self.current_token_start..self.current_token_end]
    }

    fn identifier_type(str: &str) -> TokenType {
        match str {
            "else" => TokenElse,
            "false" => TokenFalse,
            "fun" => TokenFun,
            "let" => TokenLet,
            "nil" => TokenNil,
            "if" => TokenIf,
            "repeat" => TokenRepeat,
            "return" => TokenReturn,
            "true" => TokenTrue,
            "while" => TokenWhile,
            _ => TokenIdentifier
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn match_token(scanner: &mut Scanner, token_type: TokenType, line: usize) {
        let token = scanner.next();
        assert_eq!(token_type, token.token_type);
        assert_eq!(line, token.line, "Got unexpected line");
    }

    fn match_full_token(scanner: &mut Scanner, token_type: TokenType, contents: &str, line: usize) {
        let token = scanner.next();
        assert_eq!(token_type, token.token_type);
        assert_eq!(contents, token.string);
        assert_eq!(line, token.line, "Got unexpected line");
    }

    #[test]
    fn blank_input() {
        let mut scanner = Scanner::new("");
        assert_eq!(scanner.next().token_type, EOF);
    }

    #[test]
    fn single_character_tokens() {
        let source = "(){},.-+;/*";
        let mut scanner = Scanner::new(source);

        match_token(&mut scanner, TokenLeftParen, 1);
        match_token(&mut scanner, TokenRightParen, 1);
        match_token(&mut scanner, TokenLeftBrace, 1);
        match_token(&mut scanner, TokenRightBrace, 1);
        match_token(&mut scanner, TokenComma, 1);
        match_token(&mut scanner, TokenDot, 1);
        match_token(&mut scanner, TokenMinus, 1);
        match_token(&mut scanner, TokenPlus, 1);
        match_token(&mut scanner, TokenSemicolon, 1);
        match_token(&mut scanner, TokenSlash, 1);
        match_token(&mut scanner, TokenAsterisk, 1);
        assert_eq!(scanner.next().token_type, EOF);
    }

    #[test]
    fn one_or_two_character_tokens() {
        let source = "! != = == < <= > >= && ||";
        let mut scanner = Scanner::new(source);

        match_token(&mut scanner, TokenBang, 1);
        match_token(&mut scanner, TokenBangEqual, 1);
        match_token(&mut scanner, TokenEqual, 1);
        match_token(&mut scanner, TokenEqualEqual, 1);
        match_token(&mut scanner, TokenLess, 1);
        match_token(&mut scanner, TokenLessEqual, 1);
        match_token(&mut scanner, TokenGreater, 1);
        match_token(&mut scanner, TokenGreaterEqual, 1);
        match_token(&mut scanner, TokenAmpAmp, 1);
        match_token(&mut scanner, TokenPipePipe, 1);
        assert_eq!(scanner.next().token_type, EOF);
    }

    #[test]
    fn string_literals() {
        let source = "\"Hello,\nworld!\"\n\"Hello again!!\"";
        let mut scanner = Scanner::new(source);

        match_full_token(&mut scanner, TokenString, "\"Hello,\nworld!\"", 2);
        match_full_token(&mut scanner, TokenString, "\"Hello again!!\"", 3);
        assert_eq!(scanner.next().token_type, EOF);
    }

    #[test]
    fn number_literals() {
        let source = "1 12 -13 5.55 -0.3";
        let mut scanner = Scanner::new(source);

        match_full_token(&mut scanner, TokenNumber, "1", 1);
        match_full_token(&mut scanner, TokenNumber, "12", 1);
        match_token(&mut scanner, TokenMinus, 1);
        match_full_token(&mut scanner, TokenNumber, "13", 1);
        match_full_token(&mut scanner, TokenNumber, "5.55", 1);
        match_token(&mut scanner, TokenMinus, 1);
        match_full_token(&mut scanner, TokenNumber, "0.3", 1);
        assert_eq!(scanner.next().token_type, EOF);
    }

    #[test]
    fn identifiers() {
        let source = "foo bar funky lettuce";
        let mut scanner = Scanner::new(source);

        match_full_token(&mut scanner, TokenIdentifier, "foo", 1);
        match_full_token(&mut scanner, TokenIdentifier, "bar", 1);
        match_full_token(&mut scanner, TokenIdentifier, "funky", 1);
        match_full_token(&mut scanner, TokenIdentifier, "lettuce", 1);
        assert_eq!(scanner.next().token_type, EOF);
    }

    #[test]
    fn keywords() {
        let source = "else false fun let if repeat return true while";
        let mut scanner = Scanner::new(source);

        match_full_token(&mut scanner, TokenElse, "else", 1);
        match_full_token(&mut scanner, TokenFalse, "false", 1);
        match_full_token(&mut scanner, TokenFun, "fun", 1);
        match_full_token(&mut scanner, TokenLet, "let", 1);
        match_full_token(&mut scanner, TokenIf, "if", 1);
        match_full_token(&mut scanner, TokenRepeat, "repeat", 1);
        match_full_token(&mut scanner, TokenReturn, "return", 1);
        match_full_token(&mut scanner, TokenTrue, "true", 1);
        match_full_token(&mut scanner, TokenWhile, "while", 1);
        assert_eq!(scanner.next().token_type, EOF);
    }

    #[test]
    fn comments() {
        let source = "// ignored\n==//ignored again";
        let mut scanner = Scanner::new(source);

        match_token(&mut scanner, TokenEqualEqual, 2);
        assert_eq!(scanner.next().token_type, EOF);
    }
}