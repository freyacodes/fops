use crate::bytecode::codes::{OP_EQUALS, OP_FALSE, OP_POP};
use crate::compiler::tests;

#[test]
fn expression_statement() {
    let mut code = tests::compile("false; 50 == 5;");
    tests::match_byte(&mut code, OP_FALSE);
    tests::match_byte(&mut code, OP_POP);
    tests::match_f64_op(&mut code, 50.0);
    tests::match_f64_op(&mut code, 5.0);
    tests::match_byte(&mut code, OP_EQUALS);
    tests::match_byte(&mut code, OP_POP);
    tests::assert_empty(&code);
}
