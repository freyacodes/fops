use crate::bytecode::chunk::Chunk;
use crate::bytecode::codes::*;
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

impl<'a, 'b> Parser<'a> {
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

    fn emit_bytse(&mut self, byte1: u8, byte2: u8) {
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
        if self.panic_mode { return; }
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
