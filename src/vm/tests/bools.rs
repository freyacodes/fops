use fops_macros::vm_test;
use crate::bytecode::codes::*;

#[test]
fn constants() {
    vm_test!(OP_TRUE => true);
    vm_test!(OP_FALSE => false);
}

#[test]
fn not() {
    vm_test!(OP_TRUE, OP_NOT => false);
    vm_test!(OP_FALSE, OP_NOT => true);
}

#[test]
fn equals() {
    vm_test!(OP_TRUE, OP_TRUE, OP_EQUALS => true);
    vm_test!(OP_FALSE, OP_FALSE, OP_EQUALS => true);
    vm_test!(OP_FALSE, OP_TRUE, OP_EQUALS => false);
    vm_test!(OP_TRUE, OP_FALSE, OP_EQUALS => false);
    vm_test!(OP_NIL, OP_NIL, OP_EQUALS => true);
    vm_test!(5.0, 5.0, OP_EQUALS => true);
    vm_test!(5.0, 4.0, OP_EQUALS => false);
    let nan = f64::NAN;
    vm_test!(nan, nan, OP_EQUALS => false);
}

#[test]
fn not_equals() {
    vm_test!(OP_TRUE, OP_TRUE, OP_NOT_EQUALS => false);
    vm_test!(OP_FALSE, OP_FALSE, OP_NOT_EQUALS => false);
    vm_test!(OP_FALSE, OP_TRUE, OP_NOT_EQUALS => true);
    vm_test!(OP_TRUE, OP_FALSE, OP_NOT_EQUALS => true);
    vm_test!(OP_NIL, OP_NIL, OP_NOT_EQUALS => false);
    vm_test!(5.0, 5.0, OP_NOT_EQUALS => false);
    vm_test!(5.0, 4.0, OP_NOT_EQUALS => true);
    let nan = f64::NAN;
    vm_test!(nan, nan, OP_NOT_EQUALS => true);
}