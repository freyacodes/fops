use crate::ast::operator::OperatorType;
use crate::ast::{AstExpression, AstStatement};
use crate::interpreter::stack::Stack;
use crate::interpreter::value::RuntimeValue::Boolean;
use std::collections::HashMap;
use value::RuntimeValue;

pub mod value;
mod function;
mod stack;
#[cfg(test)]
mod test;

pub struct InterpreterEndState<R> {
    pub globals: HashMap<String, RuntimeValue>,
    pub result: Result<R, String>
}

pub fn run(statements: &Vec<AstStatement>) -> InterpreterEndState<()> {
    run_with_state(statements, HashMap::new())
}

pub fn run_with_state(statements: &Vec<AstStatement>, globals: HashMap<String, RuntimeValue>) -> InterpreterEndState<()> {
    let mut stack = Stack::new(globals);
    for statement in statements { 
        let result = evaluate_statement(&mut stack, statement);
        
        if result.is_err() { 
            return InterpreterEndState {
                globals: stack.dismantle(),
                result
            }
        }
    }
    
    InterpreterEndState {
        globals: stack.dismantle(),
        result: Ok(())
    }
}

pub fn run_expression(globals: HashMap<String, RuntimeValue>, statement: &AstExpression) -> InterpreterEndState<RuntimeValue> {
    let mut stack = Stack::new(globals);
    let result = evaluate_expression(&mut stack, statement);

    InterpreterEndState {
        globals: stack.dismantle(),
        result
    }
}

fn evaluate_statement(stack: &mut Stack, statement: &AstStatement) -> Result<(), String> {
    match statement {
        AstStatement::If { conditional_clauses, else_clause } => {
            for clause in conditional_clauses {
                let condition = evaluate_expression(stack, clause.condition.as_ref())?;
                if condition == Boolean(true) {
                    return evaluate_statement(stack, clause.statement.as_ref());
                }
            }
            return match else_clause {
                None => Ok(()),
                Some(else_statement) => evaluate_statement(stack, else_statement)
            }
        },
        AstStatement::Block { statements } => { 
            stack.push_frame();
            for inner in statements {
                evaluate_statement(stack, inner)?
            }
            stack.pop_frame();
        },
        AstStatement::Expression { expression } => { evaluate_expression(stack, expression)?; },
        AstStatement::Declaration { name, expression } => { 
            let value = evaluate_expression(stack, expression)?;
            stack.declare(name.clone(), value)?;
        },
        AstStatement::Reassignment { name, expression } => {
            let value = evaluate_expression(stack, expression)?;
            stack.reassign(&name, value)?;
        }
    };
    
    Ok(())
}

fn evaluate_expression(stack: &mut Stack, element: &AstExpression) -> Result<RuntimeValue, String> {
    Ok(match element {
        AstExpression::BiOperator { operator, left, right } => {
            let left_value = evaluate_expression(stack, left)?;
            let right_value = evaluate_expression(stack, right)?;
            
            match operator {
                OperatorType::Or => todo!(),
                OperatorType::And => todo!(),
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
            let operand_value = evaluate_expression(stack, operand)?;
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
        AstExpression::FunctionCall { name, arguments } => function::invoke_function(stack, name, arguments)?,
        AstExpression::Symbol { name } => {
            match stack.get(name.as_str()) {
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

