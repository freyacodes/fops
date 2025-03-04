use crate::bytecode::chunk::Chunk;
use crate::bytecode::codes::*;
use crate::scanner::TokenType::*;
use crate::scanner::{Scanner, Token, TokenType, PLACEHOLDER_TOKEN};

pub(crate) fn compile(source: String) -> Result<Chunk, ()> {
    let mut chunk = Chunk::new();
    let mut parser = Parser::init(&source, &mut chunk);
    parser.advance();

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
        Parser {
            scanner: Scanner::new(source),
            chunk,
            current: PLACEHOLDER_TOKEN,
            previous: PLACEHOLDER_TOKEN,
            had_error: false,
            panic_mode: false,
        }
    }

    fn expression(&mut self) {
        self.parse_presedence(Precedence::PrecAssignment);
    }

    fn parse_presedence(&mut self, precedence: Precedence) {}

    fn get_rule(&self, operator_type: TokenType) -> ParseRule<'a> {
        todo!()
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
            if self.current.token_type != TokenType::ScannerError {
                break;
            } else {
                self.error_at_current(self.current.string);
            }
        }
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.chunk.write_simple(byte1);
        self.chunk.write_simple(byte2);
    }

    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write_simple(byte);
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

        eprint!("[Line {}] Error", token.line);

        if token.token_type == TokenType::EOF {
            eprint!("at end of file");
        } else if token.token_type != TokenType::ScannerError {
            eprint!(" at {}", token.string);
        }

        eprintln!("{}", message);
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
        match self.previous.string.parse::<f32>() {
            Ok(value) => self.chunk.write_constant_f32(value),
            Err(_) => self.error("Failed to parse number."),
        }
    }

    fn unary(&mut self) {
        let operator_type = self.previous.token_type;
        self.expression();

        match operator_type {
            TokenMinus => self.emit_byte(OP_NEGATE),
            _ => unreachable!(),
        }
    }
}

/// Infix parsing
impl<'a> Parser<'a> {
    fn binary(&mut self) {
        let operator_type = self.previous.token_type;
        let rule = self.get_rule(operator_type);
        self.parse_presedence(rule.precedence.next());

        match operator_type {
            TokenPlus => self.emit_byte(OP_ADD),
            TokenMinus => self.emit_byte(OP_SUBTRACT),
            TokenAsterisk => self.emit_byte(OP_MULTIPLY),
            TokenSlash => self.emit_byte(OP_DIVIDE),
            _ => unreachable!(),
        }
    }
}

impl Precedence {
    fn next(&self) -> Precedence {
        match self {
            Precedence::PrecNone => Precedence::PrecAssignment,
            Precedence::PrecAssignment => Precedence::PrecOr,
            Precedence::PrecOr => Precedence::PrecAnd,
            Precedence::PrecAnd => Precedence::PrecEquality,
            Precedence::PrecEquality => Precedence::PrecComparison,
            Precedence::PrecComparison => Precedence::PrecTerm,
            Precedence::PrecTerm => Precedence::PrecFactor,
            Precedence::PrecFactor => Precedence::PrecUnary,
            Precedence::PrecUnary => Precedence::PrecCall,
            Precedence::PrecCall => Precedence::PrecPrimary,
            Precedence::PrecPrimary => Precedence::PrecNone,
        }
    }
}
