use crate::ast::operator::OperatorType;
use crate::ast::{AstExpression, AstStatement};
use value::RuntimeValue;
use crate::interpreter::environment::Environment;

#[cfg(test)]
mod test;
mod function;
pub mod value;
pub mod environment;

pub fn start(statements: &Vec<AstStatement>) -> Result<(), String> {
    let mut environment = Environment::new();
    interpret_statements(&mut environment, statements)
}

pub fn interpret_statements(environment: &mut Environment, statements: &Vec<AstStatement>) -> Result<(), String> {
    for statement in statements {
        evaluate_statement(environment, statement)?;
    }
    
    Ok(())
}

fn evaluate_statement(environment: &mut Environment, statement: &AstStatement) -> Result<(), String> {
    match statement { 
        AstStatement::Block { statements } => { 
            for inner in statements {
                evaluate_statement(environment, inner)?
            }
        }
        AstStatement::Expression { expression } => { evaluate_expression(environment, expression)?; },
        AstStatement::Declaration { name, expression } => { 
            let value = evaluate_expression(environment, expression)?;
            environment.declare(name.clone(), value)?;
        },
        AstStatement::Reassignment { name, expression } => {
            let value = evaluate_expression(environment, expression)?;
            environment.reassign(&name, value)?;
        }
    };
    
    Ok(())
}

pub fn evaluate_expression(environment: &mut Environment, element: &AstExpression) -> Result<RuntimeValue, String> {
    Ok(match element {
        AstExpression::BiOperator { operator, left, right } => {
            let left_value = evaluate_expression(environment, left)?;
            let right_value = evaluate_expression(environment, right)?;
            
            match operator {
                OperatorType::Equality => RuntimeValue::Boolean(left_value == right_value),
                OperatorType::Inequality => RuntimeValue::Boolean(left_value != right_value),
                OperatorType::LessThan => {
                    let (l, r) = match_two_integers(&left_value, &right_value)?;
                    RuntimeValue::Boolean(l < r)
                }
                OperatorType::LessThanOrEqual => {
                    let (l, r) = match_two_integers(&left_value, &right_value)?;
                    RuntimeValue::Boolean(l <= r)
                }
                OperatorType::GreaterThan => {
                    let (l, r) = match_two_integers(&left_value, &right_value)?;
                    RuntimeValue::Boolean(l > r)
                }
                OperatorType::GreaterThanOrEqual => {
                    let (l, r) = match_two_integers(&left_value, &right_value)?;
                    RuntimeValue::Boolean(l >= r)
                }
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
        AstExpression::UnaryOperator { operator, operand } => {
            let operand_value = evaluate_expression(environment, operand)?;
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
        AstExpression::NumberLiteral { value } => {
            return if let Ok(number) = value.parse() {
                Ok(RuntimeValue::Integer(number))
            } else {
                Err(format!("Can't parse integer: {}", value))
            }
        },
        AstExpression::StringLiteral { value } => RuntimeValue::String(value.clone()),
        AstExpression::BooleanLiteral { value } => RuntimeValue::Boolean(*value),
        AstExpression::FunctionCall { name, arguments } => function::invoke_function(environment, name, arguments)?,
        AstExpression::Symbol { name } => {
            match environment.get(name.as_str()) {
                None => return Err(format!("Variable not found: {}", name)),
                Some(value) => { value.clone() }
            }
        }
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

