#[cfg(test)]
mod tests;

use crate::bytecode::chunk::Chunk;
use crate::bytecode::codes::*;
use crate::compiler::Precedence::*;
use crate::scanner::TokenType::*;
use crate::scanner::{Scanner, Token, TokenType, PLACEHOLDER_TOKEN};
use strum::VariantArray;

pub(crate) fn compile(source: String) -> Result<Chunk, ()> {
    let mut chunk = Chunk::new();
    let mut parser = Parser::init(&source, &mut chunk);

    parser.advance();
    parser.expression();
    parser.consume(EOF, "Expected end of expression.");

    match parser.had_error {
        true => Err(()),
        false => {
            parser.emit_byte(OP_RETURN);
            Ok(chunk)
        }
    }
}

struct Parser<'a> {
    current: Token<'a>,
    chunk: &'a mut Chunk,
    previous: Token<'a>,
    scanner: Scanner<'a>,
    had_error: bool,
    panic_mode: bool,
    rules: Vec<ParseRule<'a>>,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum Precedence {
    PrecNone = 0,
    PrecAssignment = 1,
    PrecOr = 2,
    PrecAnd = 3,
    PrecEquality = 4,
    PrecComparison = 5,
    PrecTerm = 6,
    PrecFactor = 7,
    PrecUnary = 8,
    PrecCall = 9,
    PrecPrimary = 10,
}

struct ParseRule<'a> {
    prefix: Option<ParseFn<'a>>,
    infix: Option<ParseFn<'a>>,
    precedence: Precedence,
}

type ParseFn<'a> = fn(&mut Parser<'a>) -> ();

impl<'a> Parser<'a> {
    fn init(source: &'a str, chunk: &'a mut Chunk) -> Parser<'a> {
        let mut parser = Parser {
            scanner: Scanner::new(source),
            chunk,
            current: PLACEHOLDER_TOKEN,
            previous: PLACEHOLDER_TOKEN,
            had_error: false,
            panic_mode: false,
            rules: Vec::new(),
        };

        parser.init_rules();
        parser
    }

    fn init_rules(&mut self) {
        TokenType::VARIANTS
            .iter()
            .for_each(|token_type: &TokenType| {
                fn rule<'a>(
                    prefix: Option<ParseFn<'a>>,
                    infix: Option<ParseFn<'a>>,
                    precedence: Precedence,
                ) -> ParseRule<'a> {
                    ParseRule {
                        prefix,
                        infix,
                        precedence,
                    }
                }

                #[rustfmt::skip]
                let rule = match token_type {
                    TokenLeftParen =>    rule(Some(Self::grouping), None,               PrecNone),
                    TokenRightParen =>   rule(None,                 None,               PrecNone),
                    TokenLeftBrace =>    rule(None,                 None,               PrecNone),
                    TokenRightBrace =>   rule(None,                 None,               PrecNone),
                    TokenComma =>        rule(None,                 None,               PrecNone),
                    TokenDot =>          rule(None,                 None,               PrecNone),
                    TokenMinus =>        rule(Some(Self::unary),    Some(Self::binary), PrecTerm),
                    TokenPlus =>         rule(None,                 Some(Self::binary), PrecTerm),
                    TokenSemicolon =>    rule(None,                 None,               PrecNone),
                    TokenSlash =>        rule(None,                 Some(Self::binary), PrecFactor),
                    TokenAsterisk =>     rule(None,                 Some(Self::binary), PrecFactor),
                    TokenBang =>         rule(Some(Self::unary),    None,               PrecNone),
                    TokenBangEqual =>    rule(None,                 Some(Self::binary), PrecEquality),
                    TokenEqual =>        rule(None,                 None,               PrecNone),
                    TokenEqualEqual =>   rule(None,                 Some(Self::binary), PrecEquality),
                    TokenLess =>         rule(None,                 Some(Self::binary), PrecComparison),
                    TokenLessEqual =>    rule(None,                 Some(Self::binary), PrecComparison),
                    TokenGreater =>      rule(None,                 Some(Self::binary), PrecComparison),
                    TokenGreaterEqual => rule(None,                 Some(Self::binary), PrecComparison),
                    TokenAmpAmp =>       rule(None,                 None,               PrecNone),
                    TokenPipePipe =>     rule(None,                 None,               PrecNone),
                    TokenIdentifier =>   rule(None,                 None,               PrecNone),
                    TokenString =>       rule(None,                 None,               PrecNone),
                    TokenNumber =>       rule(Some(Self::number),   None,               PrecNone),
                    TokenElse =>         rule(None,                 None,               PrecNone),
                    TokenFalse =>        rule(Some(Self::literal),  None,               PrecNone),
                    TokenFun =>          rule(None,                 None,               PrecNone),
                    TokenLet =>          rule(None,                 None,               PrecNone),
                    TokenNil =>          rule(Some(Self::literal),  None,               PrecNone),
                    TokenIf =>           rule(None,                 None,               PrecNone),
                    TokenRepeat =>       rule(None,                 None,               PrecNone),
                    TokenReturn =>       rule(None,                 None,               PrecNone),
                    TokenTrue =>         rule(Some(Self::literal),  None,               PrecNone),
                    TokenWhile =>        rule(None,                 None,               PrecNone),
                    EOF =>               rule(None,                 None,               PrecNone),
                    ScannerError =>      rule(None,                 None,               PrecNone),
                };

                self.rules.push(rule);
            });
    }

    fn expression(&mut self) {
        self.parse_precedence(PrecAssignment);
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let rule = self.get_rule(self.previous.token_type).prefix;

        let prefix_rule = match rule {
            None => {
                self.error("Expected expression.");
                return;
            }
            Some(prefix_rule) => prefix_rule,
        };

        prefix_rule(self);
        while precedence <= self.get_rule(self.current.token_type).precedence {
            self.advance();
            let infix_rule = self.get_rule(self.previous.token_type).infix;
            infix_rule.expect("This should only be reachable for some infix rule")(self);
        }
    }

    fn get_rule(&self, operator_type: TokenType) -> &ParseRule<'a> {
        self.rules.get(operator_type as usize).unwrap()
    }

    fn consume(&mut self, token: TokenType, message: &str) {
        if self.current.token_type == token {
            self.advance()
        } else {
            self.error_at_current(message)
        }
    }

    fn advance(&mut self) {
        self.previous = self.current;

        loop {
            self.current = self.scanner.next();
            if self.current.token_type != ScannerError {
                break;
            } else {
                self.error_at_current(self.current.string);
            }
        }
    }

    fn _emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.chunk.write(byte1, self.previous.line as u16);
        self.chunk.write(byte2, self.previous.line as u16);
    }

    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write(byte, self.previous.line as u16);
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(&self.current.clone(), message);
    }

    fn error(&mut self, message: &str) {
        self.error_at(&self.previous.clone(), message);
    }

    fn error_at(&mut self, token: &Token, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;

        eprint!("[Line {}] Error ", token.line);

        if token.token_type == EOF {
            eprint!("at end of file:");
        } else if token.token_type != ScannerError {
            eprint!("at {}: ", token.string);
        }

        eprintln!(" {}", message);
        self.had_error = true;
    }
}

/// Prefix parsing
impl<'a> Parser<'a> {
    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenRightParen, "Expected ')' after expression.");
    }

    fn number(&mut self) {
        match self.previous.string.parse::<f64>() {
            Ok(value) => self.chunk.write_constant_f64(value, self.previous.line as u16),
            Err(_) => self.error("Failed to parse number."),
        }
    }

    fn unary(&mut self) {
        let operator_type = self.previous.token_type;
        self.expression();

        match operator_type {
            TokenMinus => self.emit_byte(OP_NEGATE),
            TokenBang => self.emit_byte(OP_NOT),
            _ => unreachable!(),
        }
    }
    
    fn literal(&mut self) {
        match self.previous.token_type {
            TokenNil => self.emit_byte(OP_NIL),
            TokenTrue => self.emit_byte(OP_TRUE),
            TokenFalse => self.emit_byte(OP_FALSE),
            _ => unreachable!(),
        }
    }
}

/// Infix parsing
impl<'a> Parser<'a> {
    fn binary(&mut self) {
        let operator_type = self.previous.token_type;
        let rule = self.get_rule(operator_type);
        self.parse_precedence(rule.precedence.next());

        match operator_type {
            TokenPlus => self.emit_byte(OP_ADD),
            TokenMinus => self.emit_byte(OP_SUBTRACT),
            TokenAsterisk => self.emit_byte(OP_MULTIPLY),
            TokenSlash => self.emit_byte(OP_DIVIDE),
            TokenEqualEqual => self.emit_byte(OP_EQUALS),
            TokenBangEqual => self.emit_byte(OP_NOT_EQUALS),
            TokenLess => self.emit_byte(OP_LESS_THAN),
            TokenLessEqual => self.emit_byte(OP_LESS_THAN_OR_EQUALS),
            TokenGreater => self.emit_byte(OP_GREATER_THAN),
            TokenGreaterEqual => self.emit_byte(OP_GREATER_THAN_OR_EQUALS),
            _ => unreachable!(),
        }
    }
}

impl Precedence {
    fn next(&self) -> Precedence {
        match self {
            PrecNone => PrecAssignment,
            PrecAssignment => PrecOr,
            PrecOr => PrecAnd,
            PrecAnd => PrecEquality,
            PrecEquality => PrecComparison,
            PrecComparison => PrecTerm,
            PrecTerm => PrecFactor,
            PrecFactor => PrecUnary,
            PrecUnary => PrecCall,
            PrecCall => PrecPrimary,
            PrecPrimary => PrecNone,
        }
    }
}
