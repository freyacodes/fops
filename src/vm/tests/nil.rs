use fops_macros::vm_test;
use crate::bytecode::codes::*;
use crate::vm::value::NIL;

#[test]
fn nil_constant() {
    vm_test!(OP_NIL => NIL);
}

#[test]
fn illegal_operations() {
    vm_test!(15.0, OP_NIL, OP_ADD => !);
    vm_test!(15.0, OP_NIL, OP_SUBTRACT => !);
    vm_test!(15.0, OP_NIL, OP_MULTIPLY => !);
    vm_test!(15.0, OP_NIL, OP_DIVIDE => !);
    vm_test!(15.0, OP_NIL, OP_NOT => !);
}
