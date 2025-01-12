pub mod operator;
pub(crate) mod expression;
mod util;

use operator::OperatorType;

#[derive(PartialEq, Debug)]
pub enum AstStatement {
    Declaration { name: String, expression: Box<AstExpression> },
    Assignment { name: String, expression: Box<AstExpression> },
    FunctionCall { name: String, arguments: Vec<AstExpression> }
}

#[derive(PartialEq, Debug)]
pub enum AstExpression {
    BiOperator { operator: OperatorType, left: Box<AstExpression>, right: Box<AstExpression> },
    UnaryOperator { operator: OperatorType, operand: Box<AstExpression> },
    NumberLiteral { value: String },
    StringLiteral { value: String },
    BooleanLiteral { value: bool },
    Symbol { name: String },
}

/*
#[cfg(test)]
mod test {
    use crate::ast::operator::OperatorType::{Division, Multiplication};
    use crate::ast::AstElement::{BiOperator, If, Let, NumberLiteral, Reassignment, StringLiteral, Symbol};
    use crate::ast::{parse, AstElement};
    use crate::lexer;
    use std::vec;
    
    #[ignore]
    #[test]
    fn test_reassignment() {
        let lexed = lexer::lex_from_string("let foo = -5;".to_string()).unwrap();
        let expected = vec![
            Reassignment {
                name: "foo".to_string(),
                expression: Box::new(
                    NumberLiteral { value: "-5".to_string() }
                )
            },
        ];

        assert_eq!(parse(lexed), Ok(expected));
    }

    #[ignore]
    #[test]
    fn test_let_assignment() {
        let lexed = lexer::lex_from_string("let foo = (-500*bar)/10;".to_string()).unwrap();
        let expected = vec![
            Let {
                name: "foo".to_string(),
                expression: Box::new(
                    BiOperator {
                        operator: Division,
                        left: Box::new(BiOperator {
                            operator: Multiplication,
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

    #[ignore]
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
*/