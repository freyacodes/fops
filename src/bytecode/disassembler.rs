use crate::bytecode::codes::*;
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

        let name = INSTRUCTION_NAMES[*code as usize];
        let instruction_length = INSTRUCTION_LENGTH[*code as usize];
        let arg_from = index + 1;
        let arg_to = index + instruction_length as usize;
        let arguments = &instructions[arg_from..arg_to];
        
        match *code {
            OP_LOAD => print_f32(&index, name, arguments),
            OP_RETURN => print_simple(&index, name),
            _ => panic!("Unknown opcode {:#04x}", code),
        }
        
        index += instruction_length as usize;
    }
}

fn print_simple(index: &usize, name: &str) {
    println!("{:#04x} {}", index, name);
}

fn print_f32(index: &usize, name: &str, arg: &[u8]) {
    println!("{:#04x} {} {}", index, name, f32::from_be_bytes(arg.try_into().unwrap()));
}
