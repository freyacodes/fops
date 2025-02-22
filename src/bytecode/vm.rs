use crate::bytecode::codes;

#[allow(unused_assignments)]
pub fn run(#[allow(unused)] instructions: &Vec<u8>) {
    #[allow(unused)]
    let mut pc: usize = 0;

    macro_rules! read_byte {
        () => {{
            let byte = instructions[pc];
            pc += 1;
            byte
        }};
    }

    macro_rules! read_f32 {
        () => {
             f32::from_be_bytes([
                read_byte!(), read_byte!(), read_byte!(), read_byte!()
            ])
        }
    }

    loop {
        let instruction = read_byte!();

        match instruction {
            codes::OP_CONSTANT => {
                println!("{}", read_f32!());
                continue;
            }
            codes::OP_RETURN => {
                return;
            }
            _ => {
                panic!("Unexpected opcode: {:04x}", read_byte!());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bytecode::vm::run;

    #[test]
    fn test_program() {
        run(&vec![0x00, 0x42, 0xf6, 0x00, 0x00, 0x01])
    }
}
