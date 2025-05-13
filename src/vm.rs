pub mod value;
#[cfg(test)]
mod tests;

use crate::bytecode::chunk::Chunk;
use crate::bytecode::codes;
use crate::compiler;
use std::ops::Neg;
use crate::vm::value::{Value, FALSE, NIL, TRUE};

pub fn interpret(source: String, repl: bool) -> Result<Value, String> {
    let chunk = compiler::compile(source, repl).or(Err("Compilation failed"))?;
    run(&chunk)
}

#[allow(unused_assignments)]
pub fn run(chunk: &Chunk) -> Result<Value, String> {
    let instructions = &chunk.code;
    if instructions.is_empty() {
        return Ok(Value::Nil);
    }
    
    #[allow(unused)]
    let mut pc: usize = 0; // Performance note: This would likely be faster as a raw (unsafe) pointer
    let mut stack: Vec<Value> = Vec::new();

    macro_rules! read_byte {
        () => {{
            let byte = instructions[pc];
            pc += 1;
            byte
        }};
    }

    #[rustfmt::skip]
    macro_rules! read_f64 {
        () => {
            f64::from_be_bytes([
                read_byte!(), read_byte!(), read_byte!(), read_byte!(),
                read_byte!(), read_byte!(), read_byte!(), read_byte!()
            ])
        };
    }

    macro_rules! binary_op {
        ($operator:tt, $operation_name:literal) => {{
            let right = peek(&stack, 0).expect("Stack is empty");
            let left = peek(&stack, 1).expect("Stack only had one element");

            let result = if let (Value::Number(left), Value::Number(right)) = (left, right) {
                left $operator right
            } else {
                return runtime_error(pc, &chunk, format!("Cannot perform {} between {} and {}", $operation_name, left, right));
            };

            stack.pop().unwrap();
            stack.pop().unwrap();
            stack.push(Value::Number(result));
        }}
    }

    loop {
        let instruction: u8 = read_byte!();

        match instruction {
            codes::OP_F64 => stack.push(Value::Number(read_f64!())),
            codes::OP_NIL => stack.push(NIL),
            codes::OP_TRUE => stack.push(TRUE),
            codes::OP_FALSE => stack.push(FALSE),
            codes::OP_CONTANT => {
                let constant = chunk.load_constant(read_byte!());
                stack.push(constant);
            },

            codes::OP_ADD => {
                let right = peek(&stack, 0).expect("Stack is empty");
                let left = peek(&stack, 1).expect("Stack only had one element");

                let result = if let (Value::Number(left), Value::Number(right)) = (left, right) {
                    Value::Number(left + right)
                } else if left.is_string() || right.is_string() {
                    Value::from(format!("{}{}", left, right))
                } else {
                    return runtime_error(pc, &chunk, format!("Cannot perform addition between {} and {}", left, right));
                };

                stack.pop().unwrap();
                stack.pop().unwrap();
                stack.push(result);
            },
            codes::OP_SUBTRACT => binary_op!(-, "subtraction"),
            codes::OP_DIVIDE => binary_op!(/, "divide"),
            codes::OP_MULTIPLY => binary_op!(*, "multiplication"),
            codes::OP_NEGATE => {
                let value = stack.last_mut().expect("Stack is empty");
                match value {
                    Value::Number(number) => {
                        *number = number.neg()
                    }
                    _ => {
                        return runtime_error(pc, &chunk, format!("Attempt to negate {}", value.to_string()))
                    },
                };
            }

            codes::OP_NOT => {
                let value = stack.pop().expect("Stack is empty");
                match value {
                    Value::Bool(bool) => stack.push(Value::Bool(!bool)),
                    _ => return runtime_error(pc, &chunk, format!("Attempt to negate {}", value.to_string()))
                }
            },
            codes::OP_EQUALS => {
                let (left, right) = pop2(&mut stack);
                stack.push(Value::Bool(left == right));
            }
            codes::OP_NOT_EQUALS => {
                let (left, right) = pop2(&mut stack);
                stack.push(Value::Bool(left != right));
            }

            codes::OP_POP => { stack.pop().expect("Stack is empty"); },
            codes::OP_RETURN => return Ok(stack.pop().expect("Stack is empty")),
            _ => panic!("Unexpected opcode: {:04x}", instruction),
        }
    }
}

fn peek(stack: &Vec<Value>, offset_from_end: usize) -> Option<&Value> {
    let len = stack.len();
    stack.get(len - 1 - offset_from_end)
}

fn pop2(stack: &mut Vec<Value>) -> (Value, Value) {
    let right = stack.pop().expect("Stack is empty");
    let left = stack.pop().expect("Stack only had one element");
    (left, right)
}

fn runtime_error<T>(pc: usize, chunk: &Chunk, error: String) -> Result<T, String> {
    let line = chunk.get_line(pc - 1);
    if line == 0 {
        Err(error)
    } else {
        Err(format!("[Line {}] {}", line, error))
    }
}