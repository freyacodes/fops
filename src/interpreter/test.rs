use crate::ast::{parse_expression_only, AstStatement};
use crate::interpreter::value::RuntimeValue;
use crate::interpreter::value::RuntimeValue::{Boolean, Integer};
use crate::interpreter::{run_expression, InterpreterEndState};
use crate::{interpreter, lexer};
use std::collections::HashMap;

fn parse_statements(string: String) -> Vec<AstStatement> {
    let lexed = lexer::lex_from_string(string).unwrap();
    crate::ast::parse_script(lexed).expect("Parsing failed")
}

fn eval_expression(string: &str) -> RuntimeValue {
    let lexed = lexer::lex_from_string(string.to_string()).unwrap();
    let expression = parse_expression_only(lexed).expect("Parsing failed");
    run_expression(HashMap::new(), &expression).result.unwrap()
}

#[test]
fn test_unary() {
    assert_eq!(eval_expression("-40"), Integer(-40));
    assert_eq!(eval_expression("--40"), Integer(40));
}

#[test]
fn test_addition() {
    assert_eq!(eval_expression("5 + 2"), Integer(7));
}

#[test]
fn test_division() {
    assert_eq!(eval_expression("7 / 2"), Integer(3));
    assert_eq!(eval_expression("7 / -2"), Integer(-3));
}

#[test]
fn test_equality() {
    assert_eq!(eval_expression("5 == 2 + 3"), Boolean(true));
    assert_eq!(eval_expression("6 == 2 + 3"), Boolean(false));
    assert_eq!(eval_expression("6 == \"foo\""), Boolean(false));
}

#[test]
fn test_inequality() {
    assert_eq!(eval_expression("5 != 6"), Boolean(true));
    assert_eq!(eval_expression("5 != 2 + 3"), Boolean(false));
    assert_eq!(eval_expression("6 != 2 + 3"), Boolean(true));
    assert_eq!(eval_expression("6 != \"foo\""), Boolean(true));
}

#[test]
fn test_not_operator() {
    assert_eq!(eval_expression("!true"), Boolean(false));
    assert_eq!(eval_expression("!false"), Boolean(true));
}

#[test]
fn test_string_string_concatenation() {
    assert_eq!(
        eval_expression("\"foo\" + \"bar\""),
        RuntimeValue::String("foobar".to_string())
    );
}

#[test]
fn test_string_int_concatenation() {
    assert_eq!(
        eval_expression("\"foo\" + 5"),
        RuntimeValue::String("foo5".to_string())
    );
}

#[test]
fn test_string_boolean_concatenation() {
    assert_eq!(
        eval_expression("\"foo\" + false"),
        RuntimeValue::String("foofalse".to_string())
    );
}

#[test]
fn test_comparisons() {
    assert_eq!(eval_expression("5 < 10"), Boolean(true));
    assert_eq!(eval_expression("10 < 5"), Boolean(false));
    
    assert_eq!(eval_expression("10 > 5"), Boolean(true));
    assert_eq!(eval_expression("5 > 10"), Boolean(false));
    
    assert_eq!(eval_expression("5 <= 10"), Boolean(true));
    assert_eq!(eval_expression("10 <= 5"), Boolean(false));
    assert_eq!(eval_expression("10 <= 10"), Boolean(true));
    
    assert_eq!(eval_expression("10 >= 5"), Boolean(true));
    assert_eq!(eval_expression("5 >= 10"), Boolean(false));
    assert_eq!(eval_expression("10 >= 10"), Boolean(true));
}

#[test]
fn test_declaration() {
    let mut statements = parse_statements("let four = 4;".to_string());
    let InterpreterEndState { globals, result } = interpreter::run(&mut statements);
    result.unwrap();
    assert_eq!(globals.get(&"four".to_string()), Some(&Integer(4)));
}

#[test]
fn test_reassignment() {
    let mut statements = parse_statements("let four = 4; four = 5;".to_string());
    let InterpreterEndState { globals, result } = interpreter::run(&mut statements);
    result.unwrap();
    assert_eq!(globals.get(&"four".to_string()), Some(&Integer(5)));
}

#[test]
fn test_variable_arithmetic() {
    let mut statements = parse_statements("let minutes = 2; let seconds = minutes * 60;".to_string());
    let InterpreterEndState { globals, result } = interpreter::run(&mut statements);
    result.unwrap();
    assert_eq!(globals.get(&"seconds".to_string()), Some(&Integer(120)));
}

#[test]
fn test_group_execution() {
    let src = r#"
    let a = 1;
    {
        a = 2;
    }
    "#.to_string();
    let mut statements = parse_statements(src);
    let InterpreterEndState { globals, result } = interpreter::run(&mut statements);
    result.unwrap();
    assert_eq!(globals.get(&"a".to_string()), Some(&Integer(2)));
}

#[test]
fn test_variable_shadowing() {
    let src = r#"
    let a = 1;
    let b = 2;
    {
        let b = 3;
        a = b;
    }
    "#.to_string();
    let mut statements = parse_statements(src);
    let InterpreterEndState { globals, result } = interpreter::run(&mut statements);
    result.unwrap();
    assert_eq!(globals.get(&"a".to_string()), Some(&Integer(3)));
}
