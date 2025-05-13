use crate::bytecode::codes::{OP_EQUALS, OP_FALSE, OP_POP};
use crate::compiler::tests;

#[test]
fn expression_statements() {
    let mut code = tests::compile("false; 50 == 5;");
    tests::match_byte(&mut code, OP_FALSE);
    tests::match_byte(&mut code, OP_POP);
    tests::match_f64_op(&mut code, 50.0);
    tests::match_f64_op(&mut code, 5.0);
    tests::match_byte(&mut code, OP_EQUALS);
    tests::match_byte(&mut code, OP_POP);
    tests::assert_empty(&code);
}

#[test]
fn block_statements() {
    let mut code = tests::compile("false; { 50 == 5; } {{}}");
    tests::match_byte(&mut code, OP_FALSE);
    tests::match_byte(&mut code, OP_POP);
    tests::match_f64_op(&mut code, 50.0);
    tests::match_f64_op(&mut code, 5.0);
    tests::match_byte(&mut code, OP_EQUALS);
    tests::match_byte(&mut code, OP_POP);
    tests::assert_empty(&code);
}

#[test]
fn contiguous_locals() {
    let mut code = tests::compile("let a = 1; let b = 2;");
    tests::match_f64_op(&mut code, 1.0);
    tests::match_f64_op(&mut code, 2.0);
    tests::match_byte(&mut code, OP_POP);
    tests::match_byte(&mut code, OP_POP);
}

#[test]
fn contiguous_locals_with_block() {
    let mut code = tests::compile("let a = 1; { let b = 2; } let c = 3;");
    tests::match_f64_op(&mut code, 1.0);
    tests::match_f64_op(&mut code, 2.0);
    tests::match_byte(&mut code, OP_POP);
    tests::match_f64_op(&mut code, 3.0);
    tests::match_byte(&mut code, OP_POP);
    tests::match_byte(&mut code, OP_POP);
}
