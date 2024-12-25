use crate::lexer::Token;

#[derive(PartialEq, Debug)]
pub enum BiOperatorType {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Equality,
}

#[derive(PartialEq, Debug)]
pub enum AstElement {
    Let { name: String, assignment: Box<AstElement> },
    Assignment { statement: Box<AstElement> },
    If { condition: Box<AstElement>, then: Vec<AstElement> },
    ElseIf { condition: Box<AstElement>, then: Vec<AstElement> },
    Else { then: Vec<AstElement> },
    BiOperator { bi_operator_type: BiOperatorType, left: Box<AstElement>, right: Box<AstElement> },
    NumberLiteral { value: String },
    StringLiteral { value: String },
    FunctionCall { name: String, arguments: Vec<AstElement> },
    Symbol { name: String }
}

pub fn parse(tokens: Vec<Vec<Token>>) -> Result<Vec<AstElement>, String> {
    let elements: Vec<AstElement> = Vec::new();

    todo!()
}

#[cfg(test)]
mod test {
    use crate::ast::{parse, AstElement, BiOperatorType};
    use crate::ast::AstElement::Let;
    use crate::lexer;

    #[test]
    fn test_assignment() {
        let lexed = lexer::lex_from_string("let foo = (-500*bar)/10;".to_string()).unwrap();
        let expected = vec![
            Let {
                name: "foo".to_string(),
                assignment: Box::new(
                    AstElement::BiOperator {
                        bi_operator_type: BiOperatorType::Division,
                        left: Box::new(AstElement::BiOperator {
                            bi_operator_type: BiOperatorType::Multiplication,
                            left: Box::new(AstElement::NumberLiteral { value: "-500".to_string() }),
                            right: Box::new(AstElement::Symbol { name: "bar".to_string() })
                        }),
                        right: Box::new(AstElement::NumberLiteral { value: "10".to_string() })
                    }
                )
            },
        ];
        
        assert_eq!(parse(lexed), Ok(expected));
    }
}