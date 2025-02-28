use crate::bytecode::chunk::Chunk;
use crate::scanner::{Scanner, ScannerResult, Token};
use std::cell::Cell;

pub(crate) fn compile(source: String) -> Result<Chunk, String> {
    let mut parser = match Parser::init(source)? {
        None => return Ok(Chunk::new()),
        Some(parser) => parser,
    };

    todo!()
}

struct Parser {
    current: Cell<Token>,
    previous: Option<Token>,
    scanner: Scanner,
}

impl Parser {
    fn init(source: String) -> Result<Option<Parser>, String> {
        let mut scanner = Scanner::new(source);
        let token = match scanner.next() {
            ScannerResult::Ok(token) => token,
            ScannerResult::Err(string, _) => return Err(string),
            ScannerResult::EOF => return Ok(None),
        };

        let test = Ok(Some(Parser {
            current: Cell::new(token),
            previous: None,
            scanner,
        }));

        test
    }

    fn advance(&mut self) {
        let n = match self.scanner.next() {
            ScannerResult::Ok(next) => next,
            ScannerResult::Err(_, _) => {
                todo!()
            }
            ScannerResult::EOF => {
                todo!()
            }
        };

        let old = self.current.replace(n);
        self.previous = Some(old)
    }
}
