use crate::ast::{AstExpression, AstStatement};
use crate::interpreter::value::RuntimeValue::{Boolean, Integer};
use crate::interpreter::evaluate_expression;
use crate::{interpreter, lexer};
use crate::interpreter::environment::Environment;
use crate::interpreter::value::RuntimeValue;

fn parse_statements(string: String) -> Vec<AstStatement> {
    let lexed = lexer::lex_from_string(string).unwrap();
    crate::ast::parse_script(lexed).expect("Parsing failed")
}

fn parse_expression(string: String) -> AstExpression {
    let lexed = lexer::lex_from_string(string).unwrap();
    crate::ast::parse_expression_only(lexed).expect("Parsing failed")
}

#[test]
fn test_unary() {
    assert_eq!(evaluate_expression(&parse_expression("-40".to_string())).unwrap(), Integer(-40));
    assert_eq!(evaluate_expression(&parse_expression("--40".to_string())).unwrap(), Integer(40));
}

#[test]
fn test_addition() {
    assert_eq!(evaluate_expression(&parse_expression("5 + 2".to_string())).unwrap(), Integer(7));
}

#[test]
fn test_division() {
    assert_eq!(evaluate_expression(&parse_expression("7 / 2".to_string())).unwrap(), Integer(3));
    assert_eq!(evaluate_expression(&parse_expression("7 / -2".to_string())).unwrap(), Integer(-3));
}

#[test]
fn test_equality() {
    assert_eq!(evaluate_expression(&parse_expression("5 == 2 + 3".to_string())).unwrap(), Boolean(true));
    assert_eq!(evaluate_expression(&parse_expression("6 == 2 + 3".to_string())).unwrap(), Boolean(false));
    assert_eq!(evaluate_expression(&parse_expression("6 == \"foo\"".to_string())).unwrap(), Boolean(false));
}

#[test]
fn test_inequality() {
    assert_eq!(evaluate_expression(&parse_expression("5 != 6".to_string())).unwrap(), Boolean(true));
    assert_eq!(evaluate_expression(&parse_expression("5 != 2 + 3".to_string())).unwrap(), Boolean(false));
    assert_eq!(evaluate_expression(&parse_expression("6 != 2 + 3".to_string())).unwrap(), Boolean(true));
    assert_eq!(evaluate_expression(&parse_expression("6 != \"foo\"".to_string())).unwrap(), Boolean(true));
}

#[test]
fn test_not_operator() {
    assert_eq!(evaluate_expression(&parse_expression("!true".to_string())).unwrap(), Boolean(false));
    assert_eq!(evaluate_expression(&parse_expression("!false".to_string())).unwrap(), Boolean(true));
}

#[test]
fn test_string_string_concatenation() {
    assert_eq!(
        evaluate_expression(&parse_expression("\"foo\" + \"bar\"".to_string())).unwrap(),
        RuntimeValue::String("foobar".to_string())
    );
}

#[test]
fn test_string_int_concatenation() {
    assert_eq!(
        evaluate_expression(&parse_expression("\"foo\" + 5".to_string())).unwrap(),
        RuntimeValue::String("foo5".to_string())
    );
}

#[test]
fn test_string_boolean_concatenation() {
    assert_eq!(
        evaluate_expression(&parse_expression("\"foo\" + false".to_string())).unwrap(),
        RuntimeValue::String("foofalse".to_string())
    );
}

#[test]
fn test_comparisons() {
    assert_eq!(evaluate_expression(&parse_expression("5 < 10".to_string())).unwrap(), Boolean(true));
    assert_eq!(evaluate_expression(&parse_expression("10 < 5".to_string())).unwrap(), Boolean(false));
    
    assert_eq!(evaluate_expression(&parse_expression("10 > 5".to_string())).unwrap(), Boolean(true));
    assert_eq!(evaluate_expression(&parse_expression("5 > 10".to_string())).unwrap(), Boolean(false));
    
    assert_eq!(evaluate_expression(&parse_expression("5 <= 10".to_string())).unwrap(), Boolean(true));
    assert_eq!(evaluate_expression(&parse_expression("10 <= 5".to_string())).unwrap(), Boolean(false));
    assert_eq!(evaluate_expression(&parse_expression("10 <= 10".to_string())).unwrap(), Boolean(true));
    
    assert_eq!(evaluate_expression(&parse_expression("10 >= 5".to_string())).unwrap(), Boolean(true));
    assert_eq!(evaluate_expression(&parse_expression("5 >= 10".to_string())).unwrap(), Boolean(false));
    assert_eq!(evaluate_expression(&parse_expression("10 >= 10".to_string())).unwrap(), Boolean(true));
}

#[test]
fn test_declaration() {
    let mut statements = parse_statements("let four = 4;".to_string());
    let mut environment = Environment::new();
    interpreter::interpret_statements(&mut environment, &mut statements).unwrap();
    assert_eq!(environment.get(&"four".to_string()), Some(&RuntimeValue::Integer(4)));
}

#[test]
fn test_reassignment() {
    let mut statements = parse_statements("let four = 4; four = 5;".to_string());
    let mut environment = Environment::new();
    interpreter::interpret_statements(&mut environment, &mut statements).unwrap();
    assert_eq!(environment.get(&"four".to_string()), Some(&RuntimeValue::Integer(5)));
}