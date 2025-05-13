use crate::vm::value::Value;

mod variables;

fn assert_number(value: &Value, expected: f64) {
    assert_eq!(value, &Value::Number(expected));
}