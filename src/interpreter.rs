use crate::ast::operator::OperatorType;
use crate::ast::AstElement;
use std::cmp::PartialEq;

#[derive(Debug, PartialEq)]
pub enum RuntimeValue {
    String(String),
    Integer(i32),
    Boolean(bool)
}

impl RuntimeValue {
    pub fn value_as_string(&self) -> String {
        match self {
            RuntimeValue::String(string) => string.to_string(),
            RuntimeValue::Integer(int) => int.to_string(),
            RuntimeValue::Boolean(bool) => bool.to_string()
        }
    }
}

pub fn evaluate_expression(element: &AstElement) -> Result<RuntimeValue, String> {
    Ok(match element {
        AstElement::Let { .. } => panic!("Not intended to be used here"),
        AstElement::Reassignment { .. } => panic!("Not intended to be used here"),
        AstElement::If { .. } => panic!("Not intended to be used here"),
        AstElement::ElseIf { .. } => panic!("Not intended to be used here"),
        AstElement::Else { .. } => panic!("Not intended to be used here"),
        AstElement::BiOperator { operator, left, right } => {
            let left_value = evaluate_expression(left)?;
            let right_value = evaluate_expression(right)?;
            
            match operator {
                OperatorType::Equality => {
                    RuntimeValue::Boolean(left_value == right_value)
                },
                OperatorType::Multiplication => {
                    let (l, r) = match_two_integers(&left_value, &right_value)?;
                    RuntimeValue::Integer(l * r)
                },
                OperatorType::Division => {
                    let (l, r) = match_two_integers(&left_value, &right_value)?;
                    RuntimeValue::Integer(l / r)
                },
                OperatorType::Modulus => {
                    let (l, r) = match_two_integers(&left_value, &right_value)?;
                    RuntimeValue::Integer(l % r)
                },
                OperatorType::Plus => {
                    if let RuntimeValue::Integer(l) = left_value {
                        if let RuntimeValue::Integer(r) = right_value {
                            return Ok(RuntimeValue::Integer(l + r))
                        }
                    }

                    RuntimeValue::String(format!(
                        "{}{}",
                        left_value.value_as_string(), 
                        right_value.value_as_string())
                    )
                }
                OperatorType::Minus => {
                    let (l, r) = match_two_integers(&left_value, &right_value)?;
                    RuntimeValue::Integer(l - r)
                }
                OperatorType::Bang => unreachable!()
            }
        },
        AstElement::UnaryOperator { operator, operand } => {
            let operand_value = evaluate_expression(operand)?;
            match operator {
                OperatorType::Minus => {
                    match operand_value {
                        RuntimeValue::Integer(i) => RuntimeValue::Integer(-i),
                        _ => return error_expected_integer(&operand_value)
                    }
                },
                OperatorType::Bang => {
                    match operand_value {
                        RuntimeValue::Boolean(b) => RuntimeValue::Boolean(!b),
                        _ => return error_expected_boolean(&operand_value)
                    }
                },
                _ => panic!("Unexpected operator {}", operator)
            }
        },
        AstElement::NumberLiteral { value } => {
            return if let Ok(number) = value.parse() {
                Ok(RuntimeValue::Integer(number))
            } else {
                Err(format!("Can't parse integer: {}", value))
            }
        },
        AstElement::StringLiteral { value } => RuntimeValue::String(value.clone()),
        AstElement::BooleanLiteral { value } => RuntimeValue::Boolean(*value),
        AstElement::FunctionCall { .. } => todo!("Functions are not a thing yet"),
        AstElement::Symbol { .. } => todo!("Not implemented until variables are added")
    })
}

fn match_two_integers(left: &RuntimeValue, right: &RuntimeValue) -> Result<(i32, i32), String> {
    if let RuntimeValue::Integer(l) = left {
        if let RuntimeValue::Integer(r) = right {
            return Ok((*l, *r))
        }
    }
    
    Err(format!("Expected two integers, got {:?} and {:?}", left, right))
}

fn error_expected_integer(value: &RuntimeValue) -> Result<RuntimeValue, String> {
    Err(format!("Expected an integer, got {:?}", value))
}

fn error_expected_boolean(value: &RuntimeValue) -> Result<RuntimeValue, String> {
    Err(format!("Expected a boolean, got {:?}", value))
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;
    use crate::ast::AstElement;
    use crate::interpreter::{evaluate_expression, RuntimeValue};
    use crate::interpreter::RuntimeValue::{Boolean, Integer};
    use crate::{ast, lexer};
    use crate::lexer::Token;

    fn parse_single(string: String) -> AstElement {
        let lexed = lexer::lex_from_string(string).unwrap().into_iter().flatten().collect::<VecDeque<Token>>();
        ast::expression::parse(lexed).expect("Parsing failed")
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
}