use std::io;
use std::collections::VecDeque;
use std::io::Write;
use crate::{ast, interpreter, lexer};
use crate::interpreter::RuntimeValue;
use crate::lexer::Token;

pub fn repl() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).expect("Failure when reading stdin");

        let lexed = match lexer::lex_from_string(buffer) {
            Ok(tokens) => tokens.into_iter().collect::<VecDeque<Token>>(),
            Err(str) => { println!("Lexer error: {}", str); continue; }
        };

        let expression = match ast::parse_expression_only(lexed) {
            Ok(expression) => expression,
            Err(str) => { println!("Parser error: {}", str); continue; }
        };

        match interpreter::evaluate_expression(&expression) {
            Ok(value) => {
                match value {
                    RuntimeValue::Unit => {}
                    _ => println!("{}", value.value_as_string())
                }
            }
            Err(str) => { println!("Runtime error: {}", str); continue; }
        }
    }
}