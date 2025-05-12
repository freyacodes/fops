mod bools;
mod numbers;
mod strings;
mod nil;

use crate::vm::value::*;

fn assert_runtime_error(result: Result<Value, String>) {
    match result {
        Ok(value) => panic!("Expected runtime error, got {}", value),
        Err(_) => {}
    }
}
