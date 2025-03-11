use crate::bytecode::codes::*;
use std::collections::VecDeque;

fn compile(source: &str) -> VecDeque<u8> {
    super::compile(source.to_string()).unwrap().code.into()
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
    assert_eq!(f64::from_be_bytes(bytes), expected);
}

fn assert_empty(code: &VecDeque<u8>) {
    assert!(code.is_empty())
}

#[test]
fn negation() {
    let mut code = compile("-2");
    match_byte(&mut code, OP_CONSTANT);
    match_number(&mut code, 2.0);
    match_byte(&mut code, OP_NEGATE);
    match_byte(&mut code, OP_RETURN);
    assert_empty(&code);
}

#[test]
fn addition() {
    let mut code = compile("2 + 3");
    match_byte(&mut code, OP_CONSTANT);
    match_number(&mut code, 2.0);
    match_byte(&mut code, OP_CONSTANT);
    match_number(&mut code, 3.0);
    match_byte(&mut code, OP_ADD);
    match_byte(&mut code, OP_RETURN);
    assert_empty(&code);
}

#[test]
fn subtraction() {
    let mut code = compile("2 - 3");
    match_byte(&mut code, OP_CONSTANT);
    match_number(&mut code, 2.0);
    match_byte(&mut code, OP_CONSTANT);
    match_number(&mut code, 3.0);
    match_byte(&mut code, OP_SUBTRACT);
    match_byte(&mut code, OP_RETURN);
    assert_empty(&code);
}

#[test]
fn multiplication() {
    let mut code = compile("2 * 3");
    match_byte(&mut code, OP_CONSTANT);
    match_number(&mut code, 2.0);
    match_byte(&mut code, OP_CONSTANT);
    match_number(&mut code, 3.0);
    match_byte(&mut code, OP_MULTIPLY);
    match_byte(&mut code, OP_RETURN);
    assert_empty(&code);
}

#[test]
fn division() {
    let mut code = compile("2 + 3 / 0.5");
    match_byte(&mut code, OP_CONSTANT);
    match_number(&mut code, 2.0);
    match_byte(&mut code, OP_CONSTANT);
    match_number(&mut code, 3.0);
    match_byte(&mut code, OP_CONSTANT);
    match_number(&mut code, 0.5);
    match_byte(&mut code, OP_DIVIDE);
    match_byte(&mut code, OP_ADD);
    match_byte(&mut code, OP_RETURN);
    assert_empty(&code);
}

#[test]
fn grouping() {
    let mut code = compile("(2 + 3) / 0.5");
    match_byte(&mut code, OP_CONSTANT);
    match_number(&mut code, 2.0);
    match_byte(&mut code, OP_CONSTANT);
    match_number(&mut code, 3.0);
    match_byte(&mut code, OP_ADD);
    match_byte(&mut code, OP_CONSTANT);
    match_number(&mut code, 0.5);
    match_byte(&mut code, OP_DIVIDE);
    match_byte(&mut code, OP_RETURN);
    assert_empty(&code);
}
