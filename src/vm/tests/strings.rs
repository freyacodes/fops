use crate::bytecode::codes::*;
use fops_macros::vm_test;

#[test]
fn hello_world() {
    vm_test!("Hello, world!" => "Hello, world!");
}

#[test]
fn comparison() {
    vm_test!("Hello, world!", "Hello, world!", OP_EQUALS => true);
    vm_test!("Hello, world!", "Hello, world!", OP_NOT_EQUALS => false);
    
    vm_test!("One", "Two", OP_EQUALS => false);
    vm_test!("One", "Two", OP_NOT_EQUALS => true);
}

#[test]
fn concatenation() {
    vm_test!("Hello, ", "world!", OP_ADD => "Hello, world!");
}