use fops_macros::vm_test;
use crate::bytecode::codes::*;
use crate::vm::value::*;

fn assert_runtime_error(result: Result<Value, String>) {
    match result {
        Ok(value) => panic!("Expected runtime error, got {}", value),
        Err(_) => {}
    }
}

#[test]
fn number_constant() {
    vm_test!(123.0 => 123.0);
}

#[test]
fn nil_constant() {
    vm_test!(OP_NIL => NIL);
}

#[test]
fn bool_constants() {
    vm_test!(OP_TRUE => true);
    vm_test!(OP_FALSE => false);
}

#[test]
fn negation() {
    vm_test!(123.0, OP_NEGATE => -123.0);
}

#[test]
fn illegal_negation() {
    vm_test!(OP_FALSE, OP_NEGATE => !);
    vm_test!(OP_NIL, OP_NEGATE => !);
}

#[test]
fn addition() {
    vm_test!(15.0, 5.0, OP_ADD => 20.0);
}

#[test]
fn illegal_addition() {
    vm_test!(OP_FALSE, 15.0, OP_ADD => !);
    vm_test!(15.0, OP_FALSE, OP_ADD => !);
}

#[test]
fn subtraction() {
    vm_test!(15.0, 5.0, OP_SUBTRACT => 10.0);
}

#[test]
fn division() {
    vm_test!(15.0, 5.0, OP_DIVIDE => 3.0);
    vm_test!(15.0, 0.0, OP_DIVIDE => Value::Number(f64::INFINITY));
}

#[test]
fn multiplication() {
    vm_test!(15.0, 5.0, OP_MULTIPLY => 75.0);
}

#[test]
fn not() {
    vm_test!(OP_TRUE, OP_NOT => false);
    vm_test!(OP_FALSE, OP_NOT => true);
}

