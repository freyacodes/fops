use crate::bytecode::codes::*;
use crate::compiler::tests::*;

#[macro_export]
macro_rules! binary_operation_test {
    ($name:ident, $operator:expr, $opcode:expr) => {
        #[test]
        fn $name() {
            let mut code = crate::compiler::tests::repl_compile(format!("2 {} 3", $operator).as_str());
            crate::compiler::tests::match_f64_op(&mut code, 2.0);
            crate::compiler::tests::match_f64_op(&mut code, 3.0);
            crate::compiler::tests::match_byte(&mut code, $opcode);
            crate::compiler::tests::match_byte(&mut code, OP_RETURN);
            crate::compiler::tests::assert_empty(&code);
        }
    };
}

#[test]
fn negation() {
    let mut code = repl_compile("-2");
    match_f64_op(&mut code, 2.0);
    match_byte(&mut code, OP_NEGATE);
    match_byte(&mut code, OP_RETURN);
    assert_empty(&code);
}

binary_operation_test!(addition, "+", OP_ADD);
binary_operation_test!(subtraction, "-", OP_SUBTRACT);
binary_operation_test!(multiplication, "*", OP_MULTIPLY);

#[test]
fn division() {
    let mut code = repl_compile("2 + 3 / 0.5");
    match_f64_op(&mut code, 2.0);
    match_f64_op(&mut code, 3.0);
    match_f64_op(&mut code, 0.5);
    match_byte(&mut code, OP_DIVIDE);
    match_byte(&mut code, OP_ADD);
    match_byte(&mut code, OP_RETURN);
    assert_empty(&code);
}

#[test]
fn grouping() {
    let mut code = repl_compile("(2 + 3) / 0.5");
    match_f64_op(&mut code, 2.0);
    match_f64_op(&mut code, 3.0);
    match_byte(&mut code, OP_ADD);
    match_f64_op(&mut code, 0.5);
    match_byte(&mut code, OP_DIVIDE);
    match_byte(&mut code, OP_RETURN);
    assert_empty(&code);
}