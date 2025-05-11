use crate::bytecode::disassembler;
use std::ffi::OsStr;
use std::path::Path;
use std::{env, fs};

pub mod bytecode;
mod scanner;
pub mod vm;
mod compiler;
mod repl;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if let Some(arg) = args.get(0) {
        let path = Path::new(&arg);

        // This should probably be its own thing
        if path.extension() == Some(OsStr::new("bin")) {
            let bytes = fs::read(Box::from(path)).expect("Failed to read file");

            if env::var("DISASSEMBLE").is_ok() {
                disassembler::disassemble(bytes);
            } else {
                todo!("Fix running binary")
                /*match vm::run(&bytes.into()) {
                    Ok(value) => println!("Program completed with value {:?}", value),
                    Err(error) => eprintln!("Runtime error: {}", error),
                }*/
            }

            return
        } else {
            let string = fs::read_to_string(path).expect("Failed to read file");
            match vm::interpret(string) {
                Ok(value) => { println!("Exited with value: {}", value.to_string()); },
                Err(error) => { println!("{}", error); }
            };
        }
    } else {
        repl::start()
    }
}
