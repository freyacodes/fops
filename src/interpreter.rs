use crate::ast::AstElement;

enum RuntimeValue {
    String(String),
    Number(i64),
}

fn evaluate_expression(element: &AstElement) -> Result<RuntimeValue, String> {
    Ok(match element {
        AstElement::Let { .. } => panic!("Not intended to be used here"),
        AstElement::Reassignment { .. } => panic!("Not intended to be used here"),
        AstElement::If { .. } => panic!("Not intended to be used here"),
        AstElement::ElseIf { .. } => panic!("Not intended to be used here"),
        AstElement::Else { .. } => panic!("Not intended to be used here"),
        AstElement::BiOperator { operator, left, right } => todo!(),
        AstElement::UnaryOperator { operator, operand } => todo!(),
        AstElement::NumberLiteral { value } => RuntimeValue::Number(value.parse().unwrap()),
        AstElement::StringLiteral { value } => RuntimeValue::String(value.clone()),
        AstElement::FunctionCall { name, arguments } => todo!(),
        AstElement::Symbol { name } => todo!("Not implemented until variables are added")
    })
}