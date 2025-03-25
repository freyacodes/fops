#[cfg(test)]
mod tests {
    use crate::bytecode::chunk::Chunk;
    use crate::bytecode::codes::*;
    use crate::vm::run;
    use crate::vm::value::{Value, FALSE, NIL, TRUE};

    fn assert_number(left: f64, option_right: Result<Value, String>) {
        let right = option_right.unwrap();
        match right {
            Value::Number(number) => assert_eq!(left, number),
            _ => panic!("Expected Number {}, got {}", left, right)
        }
    }

    fn assert_runtime_error(result: Result<Value, String>) {
        match result {
            Ok(value) => panic!("Expected runtime error, got {}", value),
            Err(_) => {}
        }
    }

    #[test]
    fn constant() {
        let mut chunk = Chunk::new();
        chunk.write_constant_f64_0(123.0);
        chunk.write0(OP_RETURN);
        assert_number(123.0, run(&chunk));
    }

    fn test_literal(opcode: u8, expected: Value) {
        let mut chunk = Chunk::new();
        chunk.write0(opcode);
        chunk.write0(OP_RETURN);
        assert_eq!(expected, run(&chunk).unwrap());
    }
    
    #[test]
    fn nil_constant() {
        test_literal(OP_NIL, NIL);
    }

    #[test]
    fn bool_constants() {
        test_literal(OP_TRUE, TRUE);
        test_literal(OP_FALSE, FALSE);
    }

    #[test]
    fn negation() {
        let mut chunk = Chunk::new();
        chunk.write_constant_f64_0(123.0);
        chunk.write0(OP_NEGATE);
        chunk.write0(OP_RETURN);
        assert_number(-123.0, run(&chunk))
    }

    #[test]
    fn illegal_negation() {
        let mut chunk = Chunk::new();
        chunk.write0(OP_FALSE);
        chunk.write0(OP_NEGATE);
        chunk.write0(OP_RETURN);
        assert_runtime_error(run(&chunk));
    }

    #[test]
    fn addition() {
        let mut chunk = Chunk::new();
        chunk.write_constant_f64_0(15.0);
        chunk.write_constant_f64_0(5.0);
        chunk.write0(OP_ADD);
        chunk.write0(OP_RETURN);
        assert_number(20.0, run(&chunk))
    }

    #[test]
    fn illegal_addition() {
        let mut chunk = Chunk::new();
        chunk.write0(OP_FALSE);
        chunk.write_constant_f64_0(15.0);
        chunk.write0(OP_ADD);
        chunk.write0(OP_RETURN);
        assert_runtime_error(run(&chunk));
    }

    #[test]
    fn subtraction() {
        let mut chunk = Chunk::new();
        chunk.write_constant_f64_0(15.0);
        chunk.write_constant_f64_0(5.0);
        chunk.write0(OP_SUBTRACT);
        chunk.write0(OP_RETURN);
        assert_number(10.0, run(&chunk))
    }

    #[test]
    fn division() {
        let mut chunk = Chunk::new();
        chunk.write_constant_f64_0(15.0);
        chunk.write_constant_f64_0(5.0);
        chunk.write0(OP_DIVIDE);
        chunk.write0(OP_RETURN);
        assert_number(3.0, run(&chunk))
    }

    #[test]
    fn multiplication() {
        let mut chunk = Chunk::new();
        chunk.write_constant_f64_0(15.0);
        chunk.write_constant_f64_0(5.0);
        chunk.write0(OP_MULTIPLY);
        chunk.write0(OP_RETURN);
        assert_number(75.0, run(&chunk))
    }
}