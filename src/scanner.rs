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
    EOF,
}

#[derive(Debug, PartialEq)]
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
    TokenElse, TokenFalse, TokenFun, TokenLet,
    TokenIf, TokenReturn, TokenRepeat, TokenTrue, TokenWhile
}

impl<'a> Scanner {
    pub fn new(source: String) -> Scanner {
        Self {
            source,
            current_token_start: 0,
            current_token_end: 0,
            line: 1,
        }
    }

    pub fn next(&'a mut self) -> ScannnerResult<'a> {
        // Skip whitespace
        loop {
            if self.is_at_end() {
                return EOF;
            }
            let c = self.source.as_bytes()[self.current_token_end] as char;
            match c {
                ' ' | '\r' | '\t' => {
                    self.current_token_end += 1;
                }
                '\n' => {
                    self.line += 1;
                    self.current_token_end += 1;
                }
                _ => {
                    break;
                }
            }
        }
        
        self.current_token_start = self.current_token_end;
        
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
            '!' => {
                return if self.match_next('=') {
                    self.make_token(TokenBangEqual)
                } else {
                    self.make_token(TokenBang)
                }
            }
            '=' => {
                return if self.match_next('=') {
                    self.make_token(TokenEqualEqual)
                } else {
                    self.make_token(TokenEqual)
                }
            }
            '<' => {
                return if self.match_next('=') {
                    self.make_token(TokenLessEqual)
                } else {
                    self.make_token(TokenLess)
                }
            }
            '>' => {
                return if self.match_next('=') {
                    self.make_token(TokenGreaterEqual)
                } else {
                    self.make_token(TokenGreater)
                }
            }
            '&' => {
                return if self.match_next('&') {
                    self.make_token(TokenAmpAmp)
                } else {
                    ScannnerResult::Err("Unexpected lone &".to_string())
                }
            }
            '|' => {
                return if self.match_next('|') {
                    self.make_token(TokenPipePipe)
                } else {
                    ScannnerResult::Err("Unexpected lone |".to_string())
                }
            }
            _ => {}
        };

        todo!()
    }

    fn advance(&mut self) -> char {
        self.current_token_end += 1;
        self.source.as_bytes()[self.current_token_end - 1] as char
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.as_bytes()[self.current_token_end] as char != expected {
            return false;
        }
        self.current_token_end += 1;
        true
    }

    fn is_at_end(&self) -> bool {
        self.source.len() == self.current_token_end
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
            ScannnerResult::Ok(token) => {
                assert_eq!(token_type, token.token_type)
            }
            ScannnerResult::Err(string) => {
                panic!("Unexpected error: {}", string);
            }
            EOF => panic!("Unexpected EOF"),
        }
    }

    #[test]
    fn blank_input() {
        let source = "  \t \n".to_string();
        let mut scanner = Scanner::new(source);
        assert_eq!(scanner.next(), EOF);
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

    #[test]
    fn one_or_two_character_tokens() {
        let source = "! != = == < <= > >= && ||".to_string();
        let mut scanner = Scanner::new(source);

        match_token(&mut scanner, TokenBang);
        match_token(&mut scanner, TokenBangEqual);
        match_token(&mut scanner, TokenEqual);
        match_token(&mut scanner, TokenEqualEqual);
        match_token(&mut scanner, TokenLess);
        match_token(&mut scanner, TokenLessEqual);
        match_token(&mut scanner, TokenGreater);
        match_token(&mut scanner, TokenGreaterEqual);
        match_token(&mut scanner, TokenAmpAmp);
        match_token(&mut scanner, TokenPipePipe);
        assert_eq!(scanner.next(), EOF);
    }
}
