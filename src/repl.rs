use std::io;
use std::collections::{HashMap, VecDeque};
use std::io::Write;
use crate::{ast, interpreter, lexer};
use crate::interpreter::value::RuntimeValue;
use crate::lexer::{Token, TokenType};

pub fn repl() {
    let mut globals = HashMap::new();
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).expect("Failure when reading stdin");

        let tokens = match lexer::lex_from_string(buffer) {
            Ok(tokens) => tokens.into_iter().collect::<VecDeque<Token>>(),
            Err(str) => { println!("Lexer error: {}", str); continue; }
        };
        
        let handle_as_statements = tokens.iter().any(|token| {
            token.token_type == TokenType::Control && token.contents == ";"
        });

        globals = if handle_as_statements {
            repl_as_statements(globals, tokens)
        } else {
            repl_as_expression(globals, tokens)
        }
    }
}

fn repl_as_expression(globals: HashMap<String, RuntimeValue>, tokens: VecDeque<Token>) -> HashMap<String, RuntimeValue> {
    let expression = match ast::parse_expression_only(tokens) {
        Ok(expression) => expression,
        Err(str) => { println!("Parser error: {}", str); return globals; }
    };

    let end_state = interpreter::run_expression(globals, &expression);
    match end_state.result {
        Ok(value) => {
            match value {
                RuntimeValue::Unit => {}
                _ => println!("{}", value.value_as_string())
            }
        }
        Err(str) => { println!("Runtime error: {}", str); }
    }
    
    end_state.globals
}

fn repl_as_statements(globals: HashMap<String, RuntimeValue>, mut tokens: VecDeque<Token>) -> HashMap<String, RuntimeValue> {
    // Insert a semicolon at the end if there isn't already one
    if let Some(last_token) = tokens.back() {
        if last_token.token_type != TokenType::Control && last_token.contents != ";" {
            tokens.push_back(Token { token_type: TokenType::Control, contents: ";".to_string() });
        }
    }
    
    let statements = match ast::parse_script(tokens) {
        Ok(statements) => statements,
        Err(str) => { println!("Parser error: {}", str); return globals; }
    };
    
    let end_state = interpreter::run_with_state(&statements, globals);
    if let Err(string) = end_state.result {
        println!("Runtime error: {}", string);
    }
    end_state.globals
}