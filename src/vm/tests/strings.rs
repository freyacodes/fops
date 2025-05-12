use crate::bytecode::codes::*;
use fops_macros::vm_test;

#[test]
fn hello_world() {
    vm_test!("Hello, world!" => "Hello, world!");
}