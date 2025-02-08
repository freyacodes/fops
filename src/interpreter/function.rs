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
pub enum FunctionImplementation {
    NativeFunction { handler: Rc<dyn NativeFunctionImplementation> },
    UserFunction { statement: AstStatement },
}

impl Debug for FunctionImplementation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("<fn>")
    }
}

impl PartialEq for FunctionImplementation {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NativeFunction { handler: l }, NativeFunction { handler: r }) => {
                ptr::eq(l.as_ref(), r.as_ref())
            }
            (UserFunction { statement: l }, UserFunction { statement: r }) => l == r,
            _ => false
        }
    }
}

pub trait NativeFunctionImplementation {
    fn invoke(&self, arguments: &mut [RuntimeValue]) -> Result<RuntimeValue, String>;
}

pub(super) fn invoke_function(
    stack: &mut Stack,
    name: &str,
    arguments: &Vec<AstExpression>,
) -> Result<RuntimeValue, String> {
    match name {
        "print" | "println" => {
            if arguments.len() != 1 { return Err(format!("Expected 1 argument, got {}", arguments.len())); };
            let value = evaluate_expression(stack, arguments.get(0).unwrap())?;
            let value_string = value.value_as_string();

            if name == "println" {
                println!("{}", value_string);
            } else {
                print!("{}", value_string);
                io::stdout().flush().unwrap();
            }

            Ok(RuntimeValue::Unit)
        }
        _ => Err(format!("Function '{}' not found. Only 'print' and 'println' are currently supported", name)),
    }
}

pub(super) mod builtins {
    use crate::interpreter::function::{FunctionImplementation, NativeFunctionImplementation};
    use crate::interpreter::value::RuntimeValue;
    use std::collections::HashMap;
    use std::rc::Rc;

    pub fn initialise_globals() -> HashMap<String, RuntimeValue> {
        let mut map = HashMap::new();
        declare_native(&mut map, "print", 1, Rc::new(Print {}));
        declare_native(&mut map, "println", 1, Rc::new(Println {}));
        map
    }
    
    fn declare_native(
        map: &mut HashMap<String, RuntimeValue>,
        name: &str,
        arity: u8,
        implementation: Rc<dyn NativeFunctionImplementation>
    ) {
        map.insert(name.to_string(), RuntimeValue::Function {
            arity,
            implementation: FunctionImplementation::NativeFunction { handler: implementation },
        }).expect(format!("Failed to declare function '{}'", name).as_str());
    } 

    struct Print;
    struct Println;
    impl NativeFunctionImplementation for Println {
        fn invoke(&self, arguments: &mut [RuntimeValue]) -> Result<RuntimeValue, String> {
            let string = arguments.get(0).unwrap().value_as_string();
            println!("{}", string);
            Ok(RuntimeValue::Unit)
        }
    }

    impl NativeFunctionImplementation for Print {
        fn invoke(&self, arguments: &mut [RuntimeValue]) -> Result<RuntimeValue, String> {
            let string = arguments.get(0).unwrap().value_as_string();
            print!("{}", string);
            Ok(RuntimeValue::Unit)
        }
    }
}
