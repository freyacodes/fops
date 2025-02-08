use crate::ast::{AstExpression, AstStatement};
use crate::interpreter::evaluate_expression;
use crate::interpreter::function::FunctionImplementation::{NativeFunction, UserFunction};
use crate::interpreter::stack::Stack;
use crate::interpreter::value::RuntimeValue;
use std::fmt::{Debug, Formatter};
use std::io::Write;
use std::rc::Rc;
use std::{io, ptr};

#[derive(Clone)]
pub(super) enum FunctionImplementation {
    NativeFunction { implementation: Rc<dyn NativeFunctionImplementation> },
    UserFunction { statement: AstStatement }
}

impl Debug for FunctionImplementation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("<fn>")
    }
}

impl PartialEq for FunctionImplementation {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NativeFunction { implementation: l }, NativeFunction { implementation: r }) => {
                ptr::eq(l.as_ref(), r.as_ref())
            },
            (UserFunction { statement: l }, UserFunction { statement: r }) => l == r,
            _ => false
        }
    }
}

trait NativeFunctionImplementation {
    fn invoke(&self, arguments: &mut [RuntimeValue]);
}

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
