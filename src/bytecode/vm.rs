use crate::bytecode::codes;

#[allow(unused_assignments)]
pub fn run(#[allow(unused)] instructions: &Vec<u8>) -> f32 {
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
            codes::OP_RETURN => {
                return stack.pop().expect("Stack is empty");
            }
            _ => panic!("Unexpected opcode: {:04x}", instruction),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bytecode::vm::run;

    #[test]
    fn test_constant() {
        assert_eq!(123f32, run(&vec![0x00, 0x42, 0xf6, 0x00, 0x00, 0x01]))
    }
}
