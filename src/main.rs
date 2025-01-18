use crate::interpreter::interpret_statements;
use std::env;
use std::path::Path;

mod lexer;
mod ast;
mod interpreter;
mod repl;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    if let Some(arg) = args.get(0) {
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
        
        if let Err(error) = interpret_statements(&statements) {
            println!("Runtime error: {}", error);
        }
    } else {
        repl::repl();
    }
}
