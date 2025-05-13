mod bools;
mod numbers;
mod statements;

use crate::bytecode::codes::*;
use std::collections::VecDeque;

fn compile(source: &str) -> VecDeque<u8> {
    super::compile(source.to_string(), true).unwrap().code.into()
}

fn repl_compile(source: &str) -> VecDeque<u8> {
    super::compile(source.to_string(), true).unwrap().code.into()
}

fn match_byte(code: &mut VecDeque<u8>, byte: u8) {
    assert_eq!(code.pop_front(), Some(byte));
}

fn match_number(code: &mut VecDeque<u8>, expected: f64) {
    let bytes = [
        code.pop_front().expect("Attempt to read f64 byte 1/8"),
        code.pop_front().expect("Attempt to read f64 byte 2/8"),
        code.pop_front().expect("Attempt to read f64 byte 3/8"),
        code.pop_front().expect("Attempt to read f64 byte 4/8"),
        code.pop_front().expect("Attempt to read f64 byte 5/8"),
        code.pop_front().expect("Attempt to read f64 byte 6/8"),
        code.pop_front().expect("Attempt to read f64 byte 7/8"),
        code.pop_front().expect("Attempt to read f64 byte 8/8"),
    ];
    let actual = f64::from_be_bytes(bytes);
    if actual != expected {
        println!("Actual bytes:   {:?}", bytes);
        println!("Expected bytes: {:?}", expected.to_be_bytes());
        assert_eq!(actual, expected);
    }
}

fn match_f64_op(code: &mut VecDeque<u8>, expected: f64) {
    match_byte(code, OP_F64);
    match_number(code, expected);
}

fn assert_empty(code: &VecDeque<u8>) {
    if !code.is_empty() {
        panic!("Code is still remaining: {:?}", code);
    }
}

#[test]
fn op_literals() {
    {
        let mut code = repl_compile("nil");
        match_byte(&mut code, OP_NIL);
        match_byte(&mut code, OP_RETURN);
        assert_empty(&code);
    }
    {
        let mut code = repl_compile("true");
        match_byte(&mut code, OP_TRUE);
        match_byte(&mut code, OP_RETURN);
        assert_empty(&code);
    }
    {
        let mut code = repl_compile("false");
        match_byte(&mut code, OP_FALSE);
        match_byte(&mut code, OP_RETURN);
        assert_empty(&code);
    }
}

