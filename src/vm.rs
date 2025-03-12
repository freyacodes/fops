mod value;

use crate::bytecode::chunk::Chunk;
use crate::bytecode::codes;
use crate::compiler;
use std::ops::Neg;
use crate::vm::value::{Value, FALSE, NIL, TRUE};

pub fn interpret(source: String) -> Result<Value, String> {
    let chunk = compiler::compile(source).or(Err("Compilation failed"))?;
    run(&chunk)
}

#[allow(unused_assignments)]
pub fn run(chunk: &Chunk) -> Result<Value, String> {
    #[allow(unused)]
    let instructions = &chunk.code;
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
                return runtime_error(format!("Cannot perform {} between {} and {}", $operation_name, left, right));
            };
            
            stack.pop().unwrap();
            stack.pop().unwrap();
            stack.push(Value::Number(result));
        }}
    }

    loop {
        let instruction: u8 = read_byte!();

        match instruction {
            codes::OP_CONSTANT => stack.push(Value::Number(read_f64!())),
            codes::OP_NIL => stack.push(NIL),
            codes::OP_TRUE => stack.push(TRUE),
            codes::OP_FALSE => stack.push(FALSE),
            codes::OP_NEGATE => {
                let value = stack.last_mut().expect("Stack is empty");
                match value {
                    Value::Number(number) => {
                        *number = number.neg()
                    }
                    _ => {
                        return runtime_error(format!("Attempt to negate {}", value.to_string()))
                    },
                };
            }
            codes::OP_ADD => binary_op!(+, "addition"),
            codes::OP_SUBTRACT => binary_op!(-, "subtraction"),
            codes::OP_DIVIDE => binary_op!(/, "divide"),
            codes::OP_MULTIPLY => binary_op!(*, "multiplication"),
            codes::OP_RETURN => {
                return Ok(stack.pop().expect("Stack is empty"));
            }
            _ => panic!("Unexpected opcode: {:04x}", instruction),
        }
    }
}

fn peek(stack: &Vec<Value>, offset_from_end: usize) -> Option<&Value> {
    let len = stack.len();
    stack.get(len - 1 - offset_from_end)
}

fn runtime_error(error: String) -> Result<Value, String> {
    // TODO: Add line number
    Err(error)
}

#[cfg(test)]
mod tests {
    use crate::bytecode::chunk::Chunk;
    use crate::bytecode::codes::*;
    use crate::vm::run;
    use crate::vm::value::Value;

    fn assert_number(left: f64, option_right: Result<Value, String>) {
        let right = option_right.unwrap();
        match right {
            Value::Number(number) => assert_eq!(left, number),
            _ => panic!("Expected Number {}, got {}", left, right)
        }
    }
    
    fn assert_runtime_error(result: Result<Value, String>) {
        match result {
            Ok(value) => panic!("Expected runtime error, got {}", value),
            Err(_) => {}
        }
    }
    
    #[test]
    fn constant() {
        let mut chunk = Chunk::new();
        chunk.write_constant_f64(123.0, 0);
        chunk.write0(OP_RETURN);
        assert_number(123.0, run(&chunk));
    }

    #[test]
    fn negation() {
        let mut chunk = Chunk::new();
        chunk.write_constant_f64_0(123.0);
        chunk.write0(OP_NEGATE);
        chunk.write0(OP_RETURN);
        assert_number(-123.0, run(&chunk))
    }

    #[test]
    fn illegal_negation() {
        let mut chunk = Chunk::new();
        chunk.write0(OP_FALSE);
        chunk.write0(OP_NEGATE);
        chunk.write0(OP_RETURN);
        assert_runtime_error(run(&chunk));
    }

    #[test]
    fn addition() {
        let mut chunk = Chunk::new();
        chunk.write_constant_f64_0(15.0);
        chunk.write_constant_f64_0(5.0);
        chunk.write0(OP_ADD);
        chunk.write0(OP_RETURN);
        assert_number(20.0, run(&chunk))
    }
    
    #[test]
    fn illegal_addition() {
        let mut chunk = Chunk::new();
        chunk.write0(OP_FALSE);
        chunk.write_constant_f64_0(15.0);
        chunk.write0(OP_ADD);
        chunk.write0(OP_RETURN);
        assert_runtime_error(run(&chunk));
    }

    #[test]
    fn subtraction() {
        let mut chunk = Chunk::new();
        chunk.write_constant_f64_0(15.0);
        chunk.write_constant_f64_0(5.0);
        chunk.write0(OP_SUBTRACT);
        chunk.write0(OP_RETURN);
        assert_number(10.0, run(&chunk))
    }

    #[test]
    fn division() {
        let mut chunk = Chunk::new();
        chunk.write_constant_f64_0(15.0);
        chunk.write_constant_f64_0(5.0);
        chunk.write0(OP_DIVIDE);
        chunk.write0(OP_RETURN);
        assert_number(3.0, run(&chunk))
    }

    #[test]
    fn multiplication() {
        let mut chunk = Chunk::new();
        chunk.write_constant_f64_0(15.0);
        chunk.write_constant_f64_0(5.0);
        chunk.write0(OP_MULTIPLY);
        chunk.write0(OP_RETURN);
        assert_number(75.0, run(&chunk))
    }
}
