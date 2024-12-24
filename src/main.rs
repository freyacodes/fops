use std::path::Path;

mod lexer;
mod ast;

fn main() {
    let tokens = lexer::lex(Box::from(Path::new("enby.nb"))).unwrap();
    let ast = ast::parse(tokens).unwrap();
    println!("Hello, world!");
}
