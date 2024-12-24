use crate::lexer::Token;

pub enum BiOperatorType {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Equality,
}

pub enum AstElement {
    Let { name: String, assignment: Box<AstElement> },
    Assignment { statement: Box<AstElement> },
    If { condition: Box<AstElement>, then: Vec<AstElement> },
    ElseIf { condition: Box<AstElement>, then: Vec<AstElement> },
    Else { then: Vec<AstElement> },
    BiOperator { bi_operator_type: BiOperatorType, left: Box<AstElement>, right: Box<AstElement> },
    NumberLiteral { value: String },
    StringLiteral { value: String },
}

pub(crate) fn parse(tokens: Vec<Vec<Token>>) -> Result<Vec<AstElement>, String> {
    let elements: Vec<AstElement> = Vec::new();

    todo!()
}

