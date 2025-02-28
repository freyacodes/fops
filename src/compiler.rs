use crate::bytecode::chunk::Chunk;
use crate::scanner::Scanner;

pub(crate) fn compile(source: String) -> Result<Chunk, String> {
    let scanner = Scanner::new(source);
    todo!()
}