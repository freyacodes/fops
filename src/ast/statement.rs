use crate::ast::expression::expression;
use crate::ast::util::{consume_control, match_control, match_keyword};
use crate::ast::AstStatement::{Block, If};
use crate::ast::{AstStatement, ConditionalClause};
use crate::lexer::Token;
use crate::lexer::TokenType::{Control, Symbol};
use std::collections::VecDeque;

pub(super) fn statement(tokens: &mut VecDeque<Token>) -> Result<AstStatement, String> {
    if match_keyword(tokens, "if") { if_statement(tokens) }
    else if match_keyword(tokens, "while") { while_statement(tokens) }
    else if match_keyword(tokens, "let") { declaration_statement(tokens) }
    else if let Some(name) = match_reassignment(tokens) { reassignment_statement(tokens, name) }
    else if match_control(tokens, "{") { block_statement(tokens) }
    else { expression_statement(tokens) }
}

fn if_statement(tokens: &mut VecDeque<Token>) -> Result<AstStatement, String> {
    /// After the keywords
    fn conditional_clause(tokens: &mut VecDeque<Token>) -> Result<ConditionalClause, String> {
        consume_control(tokens, "(")?;
        let condition = expression(tokens)?;
        consume_control(tokens, ")")?;
        let statement = statement(tokens)?;
        Ok(ConditionalClause { condition, statement })
    }

    let mut conditional_clauses = vec![conditional_clause(tokens)?];

    loop {
        if !match_keyword(tokens, "else") { break; }
        if match_keyword(tokens, "if") {
            conditional_clauses.push(conditional_clause(tokens)?);
        } else {
            let else_clause = Some(Box::new(statement(tokens)?));
            return Ok(If { conditional_clauses, else_clause })
        }
    }

    Ok(If { conditional_clauses, else_clause: None })
}

fn while_statement(tokens: &mut VecDeque<Token>) -> Result<AstStatement, String> {
    consume_control(tokens, "(")?;
    let condition = expression(tokens)?;
    consume_control(tokens, ")")?;
    let statement = statement(tokens)?;
    
    Ok(AstStatement::While { condition, statement: Box::new(statement) })
}

fn block_statement(tokens: &mut VecDeque<Token>) -> Result<AstStatement, String> {
    let mut statements: Vec<AstStatement> = Vec::new();

    loop {
        let next_token = match tokens.get(0) {
            None => { return Err("Unexpected end of file after '{'".to_string()) }
            Some(token) => token
        };

        if next_token.token_type == Control && next_token.contents == "}" {
            tokens.pop_front();
            return Ok(Block { statements });
        }

        statements.push(statement(tokens)?)
    }
}

fn declaration_statement(tokens: &mut VecDeque<Token>) -> Result<AstStatement, String> {
    let name_token = match tokens.pop_front() {
        None => return Err("Expected name in let statement".to_string()),
        Some(token) => token
    };

    consume_control(tokens, "=")?;

    let statement = AstStatement::Declaration {
        name: name_token.contents,
        expression: expression(tokens)?,
    };

    consume_control(tokens, ";")?;
    Ok(statement)
}

fn match_reassignment(tokens: &mut VecDeque<Token>) -> Option<String> {
    if let Some(first) = tokens.get(0) {
        if let Some(second) = tokens.get(1) {
            if first.token_type == Symbol && second.token_type == Control && second.contents == "=" {
                let name = tokens.pop_front().unwrap();
                tokens.pop_front().unwrap(); // Drop the equals sign
                return Some(name.contents)
            }
        }
    }
    None
}

fn reassignment_statement(tokens: &mut VecDeque<Token>, name: String) -> Result<AstStatement, String> {
    let statement = AstStatement::Reassignment {
        name,
        expression: expression(tokens)?,
    };

    consume_control(tokens, ";")?;
    Ok(statement)
}

fn expression_statement(tokens: &mut VecDeque<Token>) -> Result<AstStatement, String> {
    let expression = expression(tokens)?;
    consume_control(tokens, ";")?;
    Ok(AstStatement::Expression { expression })
}

#[cfg(test)]
mod test {
    use crate::ast::statement::statement;
    use crate::ast::AstExpression::{BooleanLiteral, Call, NumberLiteral, StringLiteral, Symbol};
    use crate::ast::AstStatement::{Block, If, While};
    use crate::ast::{AstStatement, ConditionalClause};
    use crate::lexer;

    #[test]
    fn test_if_parsing() {
        let source = r#"
        if (a) {
        } else if (b) {
        } else {
        }
        "#;
        let mut lexed = lexer::lex_from_string(source.to_string()).unwrap();
        let statement = statement(&mut lexed).expect("Expected to return Ok");

        assert!(lexed.is_empty());
        assert_eq!(statement, If {
            conditional_clauses: vec![
                ConditionalClause {
                    condition: Symbol { name: "a".to_string() },
                    statement: Block { statements: vec![] },
                },
                ConditionalClause {
                    condition: Symbol { name: "b".to_string() },
                    statement: Block { statements: vec![] },
                }
            ],
            else_clause: Some(Box::new(Block { statements: vec![] })),
        });
    }
    
    #[test]
    fn test_while_parsing() {
        let source = "while (true) {}";
        let mut lexed = lexer::lex_from_string(source.to_string()).unwrap();
        let statement = statement(&mut lexed).expect("Expected to return Ok");
        
        assert!(lexed.is_empty());
        assert_eq!(statement, While {
            condition: BooleanLiteral { value: true },
            statement: Box::new(Block { statements: vec![] })
        })
    }

    #[test]
    fn test_block_statement() {
        let source = "{ test1(100); test2(200); }";
        let mut lexed = lexer::lex_from_string(source.to_string()).unwrap();
        let statement = statement(&mut lexed).expect("Expected to return Ok");

        assert!(lexed.is_empty());
        assert_eq!(statement, Block {
            statements: vec![
                AstStatement::Expression {
                    expression: Call {
                        callee: Box::new(Symbol { name: "test1".to_string() }),
                        arguments: vec![NumberLiteral { value: "100".to_string() }]
                    }
                },
                AstStatement::Expression {
                    expression: Call {
                        callee: Box::new(Symbol { name: "test2".to_string() }),
                        arguments: vec![NumberLiteral { value: "200".to_string() }]
                    }
                }
            ]
        });
    }

    #[test]
    fn test_declaration_parsing() {
        let mut lexed = lexer::lex_from_string("let four = 4;".to_string()).unwrap();
        let statement = statement(&mut lexed).expect("Expected to return Ok");

        assert_eq!(statement, AstStatement::Declaration {
            name: "four".to_string(),
            expression: NumberLiteral {
                value: "4".to_string(),
            }
        });
        assert!(lexed.is_empty(), "Expected all tokens to be fully consumed");
    }

    #[test]
    fn test_reassignment_parsing() {
        let mut lexed = lexer::lex_from_string("four = 4;".to_string()).unwrap();
        let statement = statement(&mut lexed).expect("Expected to return Ok");

        assert_eq!(statement, AstStatement::Reassignment {
            name: "four".to_string(),
            expression: NumberLiteral {
                value: "4".to_string(),
            }
        });
        assert!(lexed.is_empty(), "Expected all tokens to be fully consumed");
    }

    #[test]
    fn test_expression_parsing() {
        let mut lexed = lexer::lex_from_string("println(\"Hello, world!\");".to_string()).unwrap();
        let statement = statement(&mut lexed).expect("Expected to return Ok");

        assert_eq!(statement, AstStatement::Expression {
            expression: Call {
                callee: Box::new(Symbol { name: "println".to_string() }),
                arguments: vec![
                    StringLiteral { value: "Hello, world!".to_string() }
                ]
            }
        });
        assert!(lexed.is_empty(), "Expected all tokens to be fully consumed");
    }
}
