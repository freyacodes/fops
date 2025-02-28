use crate::bytecode::chunk::Chunk;
use crate::scanner::{Scanner, Token, TokenType, PLACEHOLDER_TOKEN};

pub(crate) fn compile(source: String) -> Result<Chunk, String> {
    let mut parser = Parser::init(&source);

    parser.advance();
    
    todo!()
}

struct Parser<'a> {
    current: Token<'a>,
    previous: Token<'a>,
    scanner: Scanner<'a>,
}

impl<'a> Parser<'a> {
    fn init(source: &'a str) -> Parser<'a> {
        Parser {
            scanner: Scanner::new(source),
            current: PLACEHOLDER_TOKEN,
            previous: PLACEHOLDER_TOKEN
        }
    }

    fn advance(&mut self) {
        self.previous = self.current;

        loop {
            self.current = self.scanner.next();
            if self.current.token_type == TokenType::TokenError {
                todo!();
            } else {
                break;
            }
        }
    }
}
