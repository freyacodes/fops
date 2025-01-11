pub mod operator;
pub(crate) mod expression;
mod util;

use operator::OperatorType;

#[derive(PartialEq, Debug)]
pub enum AstElement {
    Let { name: String, expression: Box<AstElement> },
    Reassignment { name: String, expression: Box<AstElement> },
    If { condition: Box<AstElement>, then: Vec<AstElement>, other_blocks: Vec<AstElement> },
    ElseIf { condition: Box<AstElement>, then: Vec<AstElement> },
    Else { then: Vec<AstElement> },
    BiOperator { operator: OperatorType, left: Box<AstElement>, right: Box<AstElement> },
    UnaryOperator { operator: OperatorType, operand: Box<AstElement> },
    NumberLiteral { value: String },
    StringLiteral { value: String },
    FunctionCall { name: String, arguments: Vec<AstElement> },
    Symbol { name: String },
}

/*
pub fn parse(tokens: Vec<Vec<Token>>) -> Result<Vec<AstElement>, String> {
    let mut remaining_tokens = tokens.into_iter().flatten().collect::<Vec<Token>>();
    let mut elements: Vec<AstElement> = Vec::new();

    let mut buffer: Vec<Token> = Vec::new();
    while !remaining_tokens.is_empty() {
        let next_token = remaining_tokens.first().unwrap();
        let element = match next_token {
            Let => { parse_let(&mut remaining_tokens)? }
            _ => todo!()
        };
        elements.push(element);
    }

    Ok(elements)
}

fn parse_let(remaining_tokens: &mut Vec<Token>) -> Result<AstElement, String> {
    let semicolon = remaining_tokens.iter().enumerate()
        .find(|(_, t)| t.token_type == TokenType::Special && t.contents == ";");

    let semicolon_index = match semicolon {
        // TODO: Add line number to token
        None => return Err("Missing semicolon".to_string()),
        Some((i, _)) => i
    };

    let mut tokens = remaining_tokens.drain(0..semicolon_index).collect::<VecDeque<Token>>();
    tokens.pop_front().unwrap(); // Let token that we already matched
    let symbol_name = match tokens.pop_front() {
        None => return Err("Expected variable name".to_string()),
        Some(t) => if t.token_type == TokenType::Symbol { 
            t.contents.to_string() 
        } else { 
            return Err("Expected variable name".to_string()) 
        }
    };
    
    if tokens.pop_front() != Some(Token { token_type: TokenType::Special, contents: "=".to_string() }) {
        return Err("Expected equals sign".to_string())
    }
    
    let expression = expression::parse(tokens)?;
    Ok(Let { name: symbol_name, expression: Box::new(expression) })
}

fn parse_reassignment() -> Result<AstElement, String> {
    todo!()
}

fn parse_if_blocks() -> Result<AstElement, String> {
    todo!()
}

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