use crate::bytecode::codes::*;
use crate::compiler::tests::{assert_empty, compile, match_byte};

#[test]
fn not_operator() {
    let mut code = compile("!true");
    match_byte(&mut code, OP_TRUE);
    match_byte(&mut code, OP_NOT);
    match_byte(&mut code, OP_RETURN);
    assert_empty(&code);
}