use crate::integration_tests::assert_number;
use crate::vm::value::Value;

#[test]
fn variable_assignment() {
    let src = "let a = 5; a".to_string();
    let result = crate::vm::interpret(src, true);
    assert_number(&result.unwrap(), 5.0);
}
