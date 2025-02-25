use crate::bytecode::codes;
use std::ops::Neg;
use crate::bytecode::chunk::Chunk;

#[allow(unused_assignments)]
pub fn run(chunk: &Chunk) -> f32 {
    #[allow(unused)]
    let instructions = &chunk.code;
    #[allow(unused)]
    let mut pc: usize = 0;
    let mut stack: Vec<f32> = Vec::new();

    macro_rules! read_byte {
        () => {{
            let byte = instructions[pc];
            pc += 1;
            byte
        }};
    }

    macro_rules! read_f32 {
        () => {
            f32::from_be_bytes([read_byte!(), read_byte!(), read_byte!(), read_byte!()])
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
            codes::OP_CONSTANT => stack.push(read_f32!()),
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
    use crate::bytecode::vm::run;

    #[test]
    fn test_constant() {
        let mut chunk = Chunk::new();
        chunk.write_constant_f32(123f32);
        chunk.write_simple(OP_RETURN);
        assert_eq!(123f32, run(&chunk))
    }

    #[test]
    fn test_negate() {
        let mut chunk = Chunk::new();
        chunk.write_constant_f32(123f32);
        chunk.write_simple(OP_NEGATE);
        chunk.write_simple(OP_RETURN);
        assert_eq!(-123f32, run(&chunk))
    }

    #[test]
    fn test_addition() {
        let mut chunk = Chunk::new();
        chunk.write_constant_f32(15f32);
        chunk.write_constant_f32(5f32);
        chunk.write_simple(OP_ADD);
        chunk.write_simple(OP_RETURN);
        assert_eq!(20f32, run(&chunk))
    }

    #[test]
    fn test_subtraction() {
        let mut chunk = Chunk::new();
        chunk.write_constant_f32(15f32);
        chunk.write_constant_f32(5f32);
        chunk.write_simple(OP_SUBTRACT);
        chunk.write_simple(OP_RETURN);
        assert_eq!(10f32, run(&chunk))
    }

    #[test]
    fn test_division() {
        let mut chunk = Chunk::new();
        chunk.write_constant_f32(15f32);
        chunk.write_constant_f32(5f32);
        chunk.write_simple(OP_DIVIDE);
        chunk.write_simple(OP_RETURN);
        assert_eq!(3f32, run(&chunk))
    }

    #[test]
    fn test_multiplication() {
        let mut chunk = Chunk::new();
        chunk.write_constant_f32(15f32);
        chunk.write_constant_f32(5f32);
        chunk.write_simple(OP_MULTIPLY);
        chunk.write_simple(OP_RETURN);
        assert_eq!(75f32, run(&chunk))
    }
}
