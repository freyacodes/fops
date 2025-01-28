use std::io;
use std::collections::VecDeque;
use std::io::Write;
use crate::{ast, interpreter, lexer};
use crate::interpreter::stack::Stack;
use crate::interpreter::value::RuntimeValue;
use crate::lexer::{Token, TokenType};

pub fn repl() {
    let mut stack = Stack::new();
    
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

        if handle_as_statements {
            repl_as_statements(&mut stack, tokens)
        } else {
            repl_as_expression(&mut stack, tokens)
        }
    }
}

fn repl_as_expression(mut stack: &mut Stack, tokens: VecDeque<Token>) {
    let expression = match ast::parse_expression_only(tokens) {
        Ok(expression) => expression,
        Err(str) => { println!("Parser error: {}", str); return; }
    };

    match interpreter::evaluate_expression(&mut stack, &expression) {
        Ok(value) => {
            match value {
                RuntimeValue::Unit => {}
                _ => println!("{}", value.value_as_string())
            }
        }
        Err(str) => { println!("Runtime error: {}", str); return; }
    }
}

fn repl_as_statements(mut stack: &mut Stack, mut tokens: VecDeque<Token>) {
    // Insert a semicolon at the end if there isn't already one
    if let Some(last_token) = tokens.back() {
        if last_token.token_type != TokenType::Control && last_token.contents != ";" {
            tokens.push_back(Token { token_type: TokenType::Control, contents: ";".to_string() });
        }
    }
    
    let statements = match ast::parse_script(tokens) {
        Ok(statements) => statements,
        Err(str) => { println!("Parser error: {}", str); return; }
    };
    
    let result = interpreter::interpret_statements(&mut stack, &statements);
    if let Err(string) = result {
        println!("Runtime error: {}", string);
    }
}