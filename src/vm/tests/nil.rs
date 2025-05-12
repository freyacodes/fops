use fops_macros::vm_test;
use crate::bytecode::codes::*;
use crate::vm::value::NIL;

#[test]
fn nil_constant() {
    vm_test!(OP_NIL => NIL);
}