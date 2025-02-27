use crate::scanner::ScannnerResult::EOF;
use crate::scanner::TokenType::*;

pub struct Scanner {
    source: String,
    current_token_start: usize,
    current_token_end: usize,
    line: usize,
}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    string: &'a str,
    token_type: TokenType,
    line: usize,
}

#[derive(Debug, PartialEq)]
pub enum ScannnerResult<'a> {
    Ok(Token<'a>),
    Err(String),
    EOF
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single character tokens
    TokenLeftParen, TokenRightParen, 
    TokenLeftBrace, TokenRightBrace,
    TokenComma, TokenDot, TokenMinus, TokenPlus,
    TokenSemicolon, TokenSlash, TokenAsterisk,

    // One or two character tokens
    TokenBang, TokenBangEqual,
    TokenEqual, TokenEqualEqual,
    TokenGreater, TokenGreaterEqual,
    TokenLess, TokenLessEqual,
    TokenLogicalAnd, TokenLogicalOr,

    // Literals
    TokenIdentifier, TokenString, TokenNumber,

    // Keywords
    TokenElse, TokenFalse, TokenFun, TokenLet,
    TokenIf, TokenReturn, TokenRepeat, TokenTrue, TokenWhile
}

impl<'a> Scanner {
    pub fn new(source: String) -> Scanner { Self {
        source,
        current_token_start: 0,
        current_token_end: 0,
        line: 1
    } }

    pub fn next(&'a mut self) -> ScannnerResult<'a> {
        if self.source.len() == self.current_token_end {
            return EOF
        }

        let c = self.advance();
        match c {
            '(' => return self.make_token(TokenLeftParen),
            ')' => return self.make_token(TokenRightParen),
            '{' => return self.make_token(TokenLeftBrace),
            '}' => return self.make_token(TokenRightBrace),
            ';' => return self.make_token(TokenSemicolon),
            ',' => return self.make_token(TokenComma),
            '.' => return self.make_token(TokenDot),
            '-' => return self.make_token(TokenMinus),
            '+' => return self.make_token(TokenPlus),
            '/' => return self.make_token(TokenSlash),
            '*' => return self.make_token(TokenAsterisk),
            _ => {}
        };

        todo!()
    }

    fn advance(&mut self) -> char {
        self.current_token_end += 1;
        self.source.as_bytes()[self.current_token_end - 1] as char
    }

    fn make_token(&'a mut self, token_type: TokenType) -> ScannnerResult<'a> {
        ScannnerResult::Ok(Token {
            string: &self.source.as_str()[self.current_token_start..self.current_token_end],
            token_type,
            line: self.line,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn match_token(scanner: &mut Scanner, token_type: TokenType) {
        match scanner.next() {
            ScannnerResult::Ok(token) => { assert_eq!(token_type, token.token_type) }
            ScannnerResult::Err(string) => { panic!("Unexpected error: {}", string); }
            EOF => panic!("Unexpected EOF")
        }
    }
    
    #[test]
    fn single_character_tokens() {
        let source = "(){},.-+;/*".to_string();
        let mut scanner = Scanner::new(source);
        
        match_token(&mut scanner, TokenLeftParen);
        match_token(&mut scanner, TokenRightParen);
        match_token(&mut scanner, TokenLeftBrace);
        match_token(&mut scanner, TokenRightBrace);
        match_token(&mut scanner, TokenComma);
        match_token(&mut scanner, TokenDot);
        match_token(&mut scanner, TokenMinus);
        match_token(&mut scanner, TokenPlus);
        match_token(&mut scanner, TokenSemicolon);
        match_token(&mut scanner, TokenSlash);
        match_token(&mut scanner, TokenAsterisk);
        assert_eq!(scanner.next(), EOF);
    }
    
}