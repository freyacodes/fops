mod value;

use crate::bytecode::chunk::Chunk;
use crate::bytecode::codes;
use crate::compiler;
use std::ops::Neg;

pub fn interpret(source: String) -> Result<f64, String> {
    let chunk = compiler::compile(source).or(Err("Compilation failed"))?;
    Ok(run(&chunk))
}

#[allow(unused_assignments)]
pub fn run(chunk: &Chunk) -> f64 {
    #[allow(unused)]
    let instructions = &chunk.code;
    #[allow(unused)]
    let mut pc: usize = 0; // Performance note: This would likely be faster as a raw (unsafe) pointer
    let mut stack: Vec<f64> = Vec::new();

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
        ($operator:tt) => {{
            let right = stack.pop().expect("Stack is empty");
            let left = stack.pop().expect("Stack only had one element");
            stack.push(left $operator right);
        }}
    }

    loop {
        let instruction: u8 = read_byte!();

        match instruction {
            codes::OP_CONSTANT => stack.push(read_f64!()),
            codes::OP_NEGATE => {
                let value = stack.last_mut().expect("Stack is empty");
                *value = value.neg();
            }
            codes::OP_ADD => binary_op!(+),
            codes::OP_SUBTRACT => binary_op!(-),
            codes::OP_DIVIDE => binary_op!(/),
            codes::OP_MULTIPLY => binary_op!(*),
            codes::OP_RETURN => {
                return stack.pop().expect("Stack is empty");
            }
            _ => panic!("Unexpected opcode: {:04x}", instruction),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bytecode::chunk::Chunk;
    use crate::bytecode::codes::*;
    use crate::vm::run;

    #[test]
    fn test_constant() {
        let mut chunk = Chunk::new();
        chunk.write_constant_f64(123.0);
        chunk.write_simple(OP_RETURN);
        assert_eq!(123.0, run(&chunk))
    }

    #[test]
    fn test_negate() {
        let mut chunk = Chunk::new();
        chunk.write_constant_f64(123.0);
        chunk.write_simple(OP_NEGATE);
        chunk.write_simple(OP_RETURN);
        assert_eq!(-123.0, run(&chunk))
    }

    #[test]
    fn test_addition() {
        let mut chunk = Chunk::new();
        chunk.write_constant_f64(15.0);
        chunk.write_constant_f64(5.0);
        chunk.write_simple(OP_ADD);
        chunk.write_simple(OP_RETURN);
        assert_eq!(20.0, run(&chunk))
    }

    #[test]
    fn test_subtraction() {
        let mut chunk = Chunk::new();
        chunk.write_constant_f64(15.0);
        chunk.write_constant_f64(5.0);
        chunk.write_simple(OP_SUBTRACT);
        chunk.write_simple(OP_RETURN);
        assert_eq!(10.0, run(&chunk))
    }

    #[test]
    fn test_division() {
        let mut chunk = Chunk::new();
        chunk.write_constant_f64(15.0);
        chunk.write_constant_f64(5.0);
        chunk.write_simple(OP_DIVIDE);
        chunk.write_simple(OP_RETURN);
        assert_eq!(3.0, run(&chunk))
    }

    #[test]
    fn test_multiplication() {
        let mut chunk = Chunk::new();
        chunk.write_constant_f64(15.0);
        chunk.write_constant_f64(5.0);
        chunk.write_simple(OP_MULTIPLY);
        chunk.write_simple(OP_RETURN);
        assert_eq!(75.0, run(&chunk))
    }
}
