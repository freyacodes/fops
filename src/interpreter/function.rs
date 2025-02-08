use std::io;
use std::io::Write;
use crate::ast::AstExpression;
use crate::interpreter::stack::Stack;
use crate::interpreter::evaluate_expression;
use crate::interpreter::value::RuntimeValue;

pub(super) fn invoke_function(
    stack: &mut Stack,
    name: &str,
    arguments: &Vec<AstExpression>
) -> Result<RuntimeValue, String> {
    match name { 
        "print" | "println" => {
            if arguments.len() != 1 { return Err(format!("Expected 1 argument, got {}", arguments.len())) };
            let value = evaluate_expression(stack, arguments.get(0).unwrap())?;
            let value_string = value.value_as_string();
            
            if name == "println" { 
                println!("{}", value_string);
            } else {
                print!("{}", value_string);
                io::stdout().flush().unwrap();
            }
            
            Ok(RuntimeValue::Unit)
        },
        _ => Err(format!("Function '{}' not found. Only 'print' and 'println' are currently supported", name)),
    }
}
