mod bools;

use crate::bytecode::codes::*;
use std::collections::VecDeque;

fn repl(source: &str) -> VecDeque<u8> {
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
    assert_eq!(f64::from_be_bytes(bytes), expected);
}

fn assert_empty(code: &VecDeque<u8>) {
    assert!(code.is_empty())
}

#[macro_export]
macro_rules! binary_operation_test {
    ($name:ident, $operator:expr, $opcode:expr) => {
        #[test]
        fn $name() {
            let mut code = crate::compiler::tests::repl(format!("2 {} 3", $operator).as_str());
            crate::compiler::tests::match_byte(&mut code, OP_F64);
            crate::compiler::tests::match_number(&mut code, 2.0);
            crate::compiler::tests::match_byte(&mut code, OP_F64);
            crate::compiler::tests::match_number(&mut code, 3.0);
            crate::compiler::tests::match_byte(&mut code, $opcode);
            crate::compiler::tests::match_byte(&mut code, OP_RETURN);
            crate::compiler::tests::assert_empty(&code);
        }
    };
}

#[test]
fn negation() {
    let mut code = repl("-2");
    match_byte(&mut code, OP_F64);
    match_number(&mut code, 2.0);
    match_byte(&mut code, OP_NEGATE);
    match_byte(&mut code, OP_RETURN);
    assert_empty(&code);
}

binary_operation_test!(addition, "+", OP_ADD);
binary_operation_test!(subtraction, "-", OP_SUBTRACT);
binary_operation_test!(multiplication, "*", OP_MULTIPLY);

#[test]
fn division() {
    let mut code = repl("2 + 3 / 0.5");
    match_byte(&mut code, OP_F64);
    match_number(&mut code, 2.0);
    match_byte(&mut code, OP_F64);
    match_number(&mut code, 3.0);
    match_byte(&mut code, OP_F64);
    match_number(&mut code, 0.5);
    match_byte(&mut code, OP_DIVIDE);
    match_byte(&mut code, OP_ADD);
    match_byte(&mut code, OP_RETURN);
    assert_empty(&code);
}

#[test]
fn grouping() {
    let mut code = repl("(2 + 3) / 0.5");
    match_byte(&mut code, OP_F64);
    match_number(&mut code, 2.0);
    match_byte(&mut code, OP_F64);
    match_number(&mut code, 3.0);
    match_byte(&mut code, OP_ADD);
    match_byte(&mut code, OP_F64);
    match_number(&mut code, 0.5);
    match_byte(&mut code, OP_DIVIDE);
    match_byte(&mut code, OP_RETURN);
    assert_empty(&code);
}

#[test]
fn op_literals() {
    {
        let mut code = repl("nil");
        match_byte(&mut code, OP_NIL);
        match_byte(&mut code, OP_RETURN);
        assert_empty(&code);
    }
    {
        let mut code = repl("true");
        match_byte(&mut code, OP_TRUE);
        match_byte(&mut code, OP_RETURN);
        assert_empty(&code);
    }
    {
        let mut code = repl("false");
        match_byte(&mut code, OP_FALSE);
        match_byte(&mut code, OP_RETURN);
        assert_empty(&code);
    }
}
