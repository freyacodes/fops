use crate::binary_operation_test;
use crate::bytecode::codes::*;
use crate::compiler::tests::{assert_empty, repl_compile, match_byte};

#[test]
fn not_operator() {
    let mut code = repl_compile("!true");
    match_byte(&mut code, OP_TRUE);
    match_byte(&mut code, OP_NOT);
    match_byte(&mut code, OP_RETURN);
    assert_empty(&code);
}

binary_operation_test!(equals, "==", OP_EQUALS);
binary_operation_test!(not_equals, "!=", OP_NOT_EQUALS);
binary_operation_test!(less_than, "<", OP_LESS_THAN);
binary_operation_test!(less_than_or_equals, "<=", OP_LESS_THAN_OR_EQUALS);
binary_operation_test!(greater_than, ">", OP_GREATER_THAN);
binary_operation_test!(greater_than_or_equals, ">=", OP_GREATER_THAN_OR_EQUALS);
