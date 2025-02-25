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

    loop {
        let instruction: u8 = read_byte!();

        match instruction {
            codes::OP_CONSTANT => stack.push(read_f32!()),
            codes::OP_NEGATE => {
                let value = stack.last_mut().expect("stack is empty");
                *value = value.neg();
            }
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
}
