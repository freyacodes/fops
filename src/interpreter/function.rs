use crate::ast::{AstExpression, AstStatement};
use crate::interpreter::evaluate_expression;
use crate::interpreter::function::FunctionImplementation::{NativeFunction, UserFunction};
use crate::interpreter::stack::Stack;
use crate::interpreter::value::RuntimeValue;
use std::fmt::{Debug, Formatter};
use std::ptr;
use std::rc::Rc;

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
    callee: &AstExpression,
    arguments: &Vec<AstExpression>,
) -> Result<RuntimeValue, String> {
    let resolved_callee = evaluate_expression(stack, callee)?;

    match resolved_callee {
        RuntimeValue::Function { arity, implementation } => {
            if arguments.len() != arity as usize {
                return Err(format!(
                    "Attempt to call function with {} arguments when {} were expected",
                    arguments.len(), arity
                ))
            }

            let mut resolved_arguments: Vec<RuntimeValue> = vec![];
            for expression in arguments {
                resolved_arguments.push(evaluate_expression(stack, expression)?);
            }

            match implementation {
                NativeFunction { handler } => handler.invoke(resolved_arguments.as_mut_slice()),
                UserFunction { .. } => todo!(),
            }
        }
        _ => Err(format!("Attempt to call {}", resolved_callee.type_name()))
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
        declare_native(&mut map, "len", 1, Rc::new(Len {}));
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
        });
    }

    struct Print;
    struct Println;
    struct Len;

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

    impl NativeFunctionImplementation for Len {
        fn invoke(&self, arguments: &mut [RuntimeValue]) -> Result<RuntimeValue, String> {
            let argument = arguments.get(0).unwrap();
            match argument {
                RuntimeValue::String(string) => { Ok(RuntimeValue::Integer(string.len() as i32)) }
                _ => Err(format!("Attempt to get length of {}", argument.type_name()))
            }
        }
    }
}
