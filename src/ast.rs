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
    If { condition: Box<AstElement>, then: Vec<AstElement>, other_blocks: Vec<AstElement> },
    ElseIf { condition: Box<AstElement>, then: Vec<AstElement> },
    Else { then: Vec<AstElement> },
    BiOperator { bi_operator_type: BiOperatorType, left: Box<AstElement>, right: Box<AstElement> },
    NumberLiteral { value: String },
    StringLiteral { value: String },
    FunctionCall { name: String, arguments: Vec<AstElement> },
    Symbol { name: String },
}

pub fn parse(tokens: Vec<Vec<Token>>) -> Result<Vec<AstElement>, String> {
    let elements: Vec<AstElement> = Vec::new();

    todo!()
}

#[cfg(test)]
mod test {
    use std::vec;
    use crate::ast::{parse, AstElement, BiOperatorType};
    use crate::ast::AstElement::{If, Let, StringLiteral};
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

    #[test]
    fn test_if_else_then() {
        let source = include_str!("test/ast_if_elseif_else.enby").to_string();
        let lexed = lexer::lex_from_string(source).unwrap();

        let expected = vec![
            If {
                condition: Box::new(AstElement::Symbol { name: "foo".to_string() }),
                then: vec![AstElement::FunctionCall {
                    name: "println".to_string(),
                    arguments: vec![
                        StringLiteral { value: "foo".to_string() }
                    ]
                }],
                other_blocks: vec![
                    AstElement::ElseIf {
                        condition: Box::new(AstElement::Symbol { name: "bar".to_string() }),
                        then: vec![AstElement::FunctionCall {
                            name: "println".to_string(),
                            arguments: vec![
                                StringLiteral { value: "bar".to_string() }
                            ]
                        }]
                    },
                    AstElement::Else {
                        then: vec![AstElement::FunctionCall {
                            name: "println".to_string(),
                            arguments: vec![
                                StringLiteral { value: "baz".to_string() }
                            ]
                        }]
                    }
                ],
            }
        ];

        assert_eq!(parse(lexed), Ok(expected));
    }
}