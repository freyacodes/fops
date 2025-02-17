use crate::bytecode::codes::{INSTRUCTION_COUNT, INSTRUCTION_LENGTH, INSTRUCTION_NAMES};
use std::fs;
use std::path::Path;

pub fn disassemble_file(file: Box<Path>) {
    let instructions = fs::read(file).unwrap();

    let mut index = 0;
    while index < instructions.len() {
        let code = &instructions[index];
        if code >= &INSTRUCTION_COUNT {
            panic!("Unknown instruction {:#x}", code);
        }

        let name = INSTRUCTION_NAMES[index];
        index += INSTRUCTION_LENGTH[index] as usize;
        println!("{:#04x} {}", index, name);
    }
}
