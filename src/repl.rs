use std::io;
use std::io::Write;
use crate::vm;

pub fn start() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).expect("Failure while reading stdin");

        match vm::interpret(buffer, true) {
            Ok(value) => println!("{}", value.to_string()),
            Err(error) => println!("Runtime error: {}", error),
        }
    }
}