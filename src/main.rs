use crate::bytecode::disassembler;
use std::ffi::OsStr;
use std::path::Path;
use std::{env, fs};

pub mod bytecode;
mod scanner;
pub mod vm;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if let Some(arg) = args.get(0) {
        let path = Path::new(&arg);

        // This should probably be its own thing
        if path.extension() == Some(OsStr::new("bin")) {
            let bytes = fs::read(Box::from(path)).unwrap();

            if env::var("DISASSEMBLE").is_ok() {
                disassembler::disassemble(bytes);
            } else {
                let code = vm::run(&bytes.into());
                println!("Program completed with code {}", code);
            }

            return
        }

        println!("Normal execution would go here, but it is not ready yet")
    } else {
        println!("The REPL would go here, but it is removed for now")
    }
}
