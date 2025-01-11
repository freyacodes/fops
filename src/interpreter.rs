use crate::ast::operator::OperatorType;
use crate::ast::AstElement;
use std::cmp::PartialEq;

#[derive(Debug, PartialEq)]
pub enum RuntimeValue {
    String(String),
    Integer(i32),
    Boolean(bool)
}

fn evaluate_expression(element: &AstElement) -> Result<RuntimeValue, String> {
    Ok(match element {
        AstElement::Let { .. } => panic!("Not intended to be used here"),
        AstElement::Reassignment { .. } => panic!("Not intended to be used here"),
        AstElement::If { .. } => panic!("Not intended to be used here"),
        AstElement::ElseIf { .. } => panic!("Not intended to be used here"),
        AstElement::Else { .. } => panic!("Not intended to be used here"),
        AstElement::BiOperator { operator, left, right } => todo!(),
        AstElement::UnaryOperator { operator, operand } => {
            if operator != &OperatorType::Subtraction { panic!("Unexpected operator {}", operator) }
            let operand_value = evaluate_expression(operand)?;
            match operand_value {
                RuntimeValue::Integer(i) => RuntimeValue::Integer(-i),
                _ => return error_expected_integer(&operand_value)
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
        AstElement::FunctionCall { name, arguments } => todo!("Functions are not a thing yet"),
        AstElement::Symbol { name } => todo!("Not implemented until variables are added")
    })
}

fn error_expected_integer(value: &RuntimeValue) -> Result<RuntimeValue, String> {
    Err(format!("Expected an integer, got {:?}", value))
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;
    use crate::ast::AstElement;
    use crate::interpreter::evaluate_expression;
    use crate::interpreter::RuntimeValue::Integer;
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
}