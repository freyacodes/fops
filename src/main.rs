use crate::bytecode::disassembler::disassemble_file;
use std::env;
use std::ffi::OsStr;
use std::path::Path;

mod lexer;
mod ast;
mod interpreter;
mod repl;
pub mod bytecode;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if let Some(arg) = args.get(0) {
        let path = Path::new(&arg);

        // This should probably be its own thing
        if path.extension() == Some(OsStr::new("bin")) {
            disassemble_file(Box::from(path));
            return
        }

        let lexed = match lexer::lex_from_file(Box::from(Path::new(&arg))) {
            Ok(lexed) => lexed,
            Err(err) => {
                println!("Lexer: {}", err);
                return;
            }
        };

        let statements = match ast::parse_script(lexed) {
            Ok(statements) => statements,
            Err(err) => {
                println!("Syntax error: {}", err);
                return;
            }
        };

        if let Err(error) = interpreter::run(&statements).result {
            println!("Runtime error: {}", error);
        }
    } else {
        repl::repl();
    }
}
