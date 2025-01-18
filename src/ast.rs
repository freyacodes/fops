pub mod operator;
mod expression;
mod util;
mod statement;

use operator::OperatorType;
use std::collections::VecDeque;
use crate::lexer::Token;

#[derive(PartialEq, Debug)]
pub enum AstStatement {
    Declaration { name: String, expression: Box<AstExpression> },
    Reassignment { name: String, expression: Box<AstExpression> },
    Expression { expression: Box<AstExpression> }
}

#[derive(PartialEq, Debug)]
pub enum AstExpression {
    BiOperator { operator: OperatorType, left: Box<AstExpression>, right: Box<AstExpression> },
    UnaryOperator { operator: OperatorType, operand: Box<AstExpression> },
    NumberLiteral { value: String },
    StringLiteral { value: String },
    BooleanLiteral { value: bool },
    Symbol { name: String },
    FunctionCall { name: String, arguments: Vec<AstExpression> }
}

pub fn parse_script(mut tokens: VecDeque<Token>) -> Result<Vec<AstStatement>, String> {
    let mut statements: Vec<AstStatement> = Vec::new();

    while !tokens.is_empty() {
        statements.push(statement::statement(&mut tokens)?)
    }
    
    Ok(statements)
}

pub fn parse_expression_only(mut tokens: VecDeque<Token>) -> Result<AstExpression, String> {
    expression::expression(&mut tokens)
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
