use crate::ast::expression::expression;
use crate::ast::util::consume_control;
use crate::ast::AstStatement;
use crate::ast::AstStatement::Block;
use crate::lexer::Token;
use crate::lexer::TokenType::{Control, Keyword, Symbol};
use std::collections::VecDeque;

pub(super) fn statement(tokens: &mut VecDeque<Token>) -> Result<AstStatement, String> {
    block_statement(tokens)
}

fn block_statement(tokens: &mut VecDeque<Token>) -> Result<AstStatement, String> {
    if let Some(first) = tokens.get(0) {
        if first.token_type == Control && first.contents == "{" {
            tokens.pop_front();
            let mut statements: Vec<AstStatement> = Vec::new();

            loop {
                let next_token = match tokens.get(0) {
                    None => { return Err("Unexpected end of file after '{'".to_string()) }
                    Some(token) => token
                };

                if next_token.token_type == Control && next_token.contents == "}" {
                    tokens.pop_front();
                    return Ok(Block { statements })
                }
                
                statements.push(statement(tokens)?)
            }
        }
    }

    declaration_statement(tokens)
}

fn declaration_statement(tokens: &mut VecDeque<Token>) -> Result<AstStatement, String> {
    if let Some(first) = tokens.get(0) {
        if first.token_type == Keyword && first.contents == "let" {
            tokens.pop_front();
            let name_token = match tokens.pop_front() {
                None => return Err("Expected name in let statement".to_string()),
                Some(token) => token
            };

            consume_control(tokens, "=")?;

            let statement = AstStatement::Declaration {
                name: name_token.contents,
                expression: Box::new(expression(tokens)?),
            };

            consume_control(tokens, ";")?;
            return Ok(statement);
        }
    }

    reassignment_statement(tokens)
}

fn reassignment_statement(tokens: &mut VecDeque<Token>) -> Result<AstStatement, String> {
    if let Some(first) = tokens.get(0) {
        if let Some(second) = tokens.get(1) {
            if first.token_type == Symbol && second.token_type == Control && second.contents == "=" {
                let name_token = tokens.pop_front().unwrap();
                tokens.pop_front(); // Drop the =

                let statement = AstStatement::Reassignment {
                    name: name_token.contents,
                    expression: Box::new(expression(tokens)?),
                };

                consume_control(tokens, ";")?;
                return Ok(statement);
            }
        }
    }

    expression_statement(tokens)
}

fn expression_statement(tokens: &mut VecDeque<Token>) -> Result<AstStatement, String> {
    let expression = expression(tokens)?;
    consume_control(tokens, ";")?;
    Ok(AstStatement::Expression { expression: Box::new(expression) })
}

#[cfg(test)]
mod test {
    use crate::ast::statement::statement;
    use crate::ast::AstExpression::{FunctionCall, NumberLiteral, StringLiteral};
    use crate::ast::AstStatement;
    use crate::lexer;

    #[test]
    fn test_block_statement() {
        let source = "{ test1(100); test2(200); }";
        let mut lexed = lexer::lex_from_string(source.to_string()).unwrap();
        let statement = statement(&mut lexed).expect("Expected to return Ok");

        assert!(lexed.is_empty());
        assert_eq!(statement, AstStatement::Block {
            statements: vec![
                AstStatement::Expression {
                    expression: Box::new(FunctionCall {
                        name: "test1".to_string(),
                        arguments: vec![NumberLiteral { value: "100".to_string() }]
                    })
                },
                AstStatement::Expression {
                    expression: Box::new(FunctionCall {
                        name: "test2".to_string(),
                        arguments: vec![NumberLiteral { value: "200".to_string() }]
                    })
                }
            ]
        });
    }

    #[test]
    fn test_declaration_statement() {
        let mut lexed = lexer::lex_from_string("let four = 4;".to_string()).unwrap();
        let statement = statement(&mut lexed).expect("Expected to return Ok");

        assert_eq!(statement, AstStatement::Declaration {
            name: "four".to_string(),
            expression: Box::new(NumberLiteral {
                value: "4".to_string(),
            })
        });
        assert!(lexed.is_empty(), "Expected all tokens to be fully consumed");
    }

    #[test]
    fn test_reassignment_statement() {
        let mut lexed = lexer::lex_from_string("four = 4;".to_string()).unwrap();
        let statement = statement(&mut lexed).expect("Expected to return Ok");

        assert_eq!(statement, AstStatement::Reassignment {
            name: "four".to_string(),
            expression: Box::new(NumberLiteral {
                value: "4".to_string(),
            })
        });
        assert!(lexed.is_empty(), "Expected all tokens to be fully consumed");
    }

    #[test]
    fn test_expression_statement() {
        let mut lexed = lexer::lex_from_string("println(\"Hello, world!\");".to_string()).unwrap();
        let statement = statement(&mut lexed).expect("Expected to return Ok");

        assert_eq!(statement, AstStatement::Expression {
            expression: Box::new(FunctionCall {
                name: "println".to_string(),
                arguments: vec![
                    StringLiteral { value: "Hello, world!".to_string() }
                ]
            })
        });
        assert!(lexed.is_empty(), "Expected all tokens to be fully consumed");
    }
}