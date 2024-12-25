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
    Let { name: String, statement: Box<AstElement> },
    Reassignment { name: String, statement: Box<AstElement> },
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
    use BiOperatorType::{Division, Multiplication};
    use crate::ast::{parse, AstElement, BiOperatorType};
    use crate::ast::AstElement::{BiOperator, If, Let, NumberLiteral, Reassignment, StringLiteral, Symbol};
    use crate::lexer;

    #[test]
    fn test_reassignment() {
        let lexed = lexer::lex_from_string("let foo = -5;".to_string()).unwrap();
        let expected = vec![
            Reassignment {
                name: "foo".to_string(),
                statement: Box::new(
                    NumberLiteral { value: "-5".to_string() }
                )
            },
        ];

        assert_eq!(parse(lexed), Ok(expected));
    }

    #[test]
    fn test_let_assignment() {
        let lexed = lexer::lex_from_string("let foo = (-500*bar)/10;".to_string()).unwrap();
        let expected = vec![
            Let {
                name: "foo".to_string(),
                statement: Box::new(
                    BiOperator {
                        bi_operator_type: Division,
                        left: Box::new(BiOperator {
                            bi_operator_type: Multiplication,
                            left: Box::new(NumberLiteral { value: "-500".to_string() }),
                            right: Box::new(Symbol { name: "bar".to_string() })
                        }),
                        right: Box::new(NumberLiteral { value: "10".to_string() })
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
                condition: Box::new(Symbol { name: "foo".to_string() }),
                then: vec![AstElement::FunctionCall {
                    name: "println".to_string(),
                    arguments: vec![
                        StringLiteral { value: "foo".to_string() }
                    ]
                }],
                other_blocks: vec![
                    AstElement::ElseIf {
                        condition: Box::new(Symbol { name: "bar".to_string() }),
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