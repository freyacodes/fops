use crate::ast::AstExpression;
use crate::interpreter::value::RuntimeValue::{Boolean, Integer};
use crate::interpreter::evaluate_expression;
use crate::lexer;
use crate::interpreter::value::RuntimeValue;

fn parse_single(string: String) -> AstExpression {
    let lexed = lexer::lex_from_string(string).unwrap();
    crate::ast::parse_expression_only(lexed).expect("Parsing failed")
}

#[test]
fn test_unary() {
    assert_eq!(evaluate_expression(&parse_single("-40".to_string())).unwrap(), Integer(-40));
    assert_eq!(evaluate_expression(&parse_single("--40".to_string())).unwrap(), Integer(40));
}

#[test]
fn test_addition() {
    assert_eq!(evaluate_expression(&parse_single("5 + 2".to_string())).unwrap(), Integer(7));
}

#[test]
fn test_division() {
    assert_eq!(evaluate_expression(&parse_single("7 / 2".to_string())).unwrap(), Integer(3));
    assert_eq!(evaluate_expression(&parse_single("7 / -2".to_string())).unwrap(), Integer(-3));
}

#[test]
fn test_equality() {
    assert_eq!(evaluate_expression(&parse_single("5 == 2 + 3".to_string())).unwrap(), Boolean(true));
    assert_eq!(evaluate_expression(&parse_single("6 == 2 + 3".to_string())).unwrap(), Boolean(false));
    assert_eq!(evaluate_expression(&parse_single("6 == \"foo\"".to_string())).unwrap(), Boolean(false));
}

#[test]
fn test_inequality() {
    assert_eq!(evaluate_expression(&parse_single("5 != 6".to_string())).unwrap(), Boolean(true));
    assert_eq!(evaluate_expression(&parse_single("5 != 2 + 3".to_string())).unwrap(), Boolean(false));
    assert_eq!(evaluate_expression(&parse_single("6 != 2 + 3".to_string())).unwrap(), Boolean(true));
    assert_eq!(evaluate_expression(&parse_single("6 != \"foo\"".to_string())).unwrap(), Boolean(true));
}

#[test]
fn test_not_operator() {
    assert_eq!(evaluate_expression(&parse_single("!true".to_string())).unwrap(), Boolean(false));
    assert_eq!(evaluate_expression(&parse_single("!false".to_string())).unwrap(), Boolean(true));
}

#[test]
fn test_string_string_concatenation() {
    assert_eq!(
        evaluate_expression(&parse_single("\"foo\" + \"bar\"".to_string())).unwrap(),
        RuntimeValue::String("foobar".to_string())
    );
}

#[test]
fn test_string_int_concatenation() {
    assert_eq!(
        evaluate_expression(&parse_single("\"foo\" + 5".to_string())).unwrap(),
        RuntimeValue::String("foo5".to_string())
    );
}

#[test]
fn test_string_boolean_concatenation() {
    assert_eq!(
        evaluate_expression(&parse_single("\"foo\" + false".to_string())).unwrap(),
        RuntimeValue::String("foofalse".to_string())
    );
}

#[test]
fn test_comparisons() {
    assert_eq!(evaluate_expression(&parse_single("5 < 10".to_string())).unwrap(), Boolean(true));
    assert_eq!(evaluate_expression(&parse_single("10 < 5".to_string())).unwrap(), Boolean(false));


    assert_eq!(evaluate_expression(&parse_single("10 > 5".to_string())).unwrap(), Boolean(true));
    assert_eq!(evaluate_expression(&parse_single("5 > 10".to_string())).unwrap(), Boolean(false));


    assert_eq!(evaluate_expression(&parse_single("5 <= 10".to_string())).unwrap(), Boolean(true));
    assert_eq!(evaluate_expression(&parse_single("10 <= 5".to_string())).unwrap(), Boolean(false));
    assert_eq!(evaluate_expression(&parse_single("10 <= 10".to_string())).unwrap(), Boolean(true));


    assert_eq!(evaluate_expression(&parse_single("10 >= 5".to_string())).unwrap(), Boolean(true));
    assert_eq!(evaluate_expression(&parse_single("5 >= 10".to_string())).unwrap(), Boolean(false));
    assert_eq!(evaluate_expression(&parse_single("10 >= 10".to_string())).unwrap(), Boolean(true));
}