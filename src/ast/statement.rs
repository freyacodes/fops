use crate::ast::expression::expression;
use crate::ast::util::consume_control;
use crate::ast::AstStatement;
use crate::lexer::Token;
use std::collections::VecDeque;

pub(super) fn statement(tokens: &mut VecDeque<Token>) -> Result<AstStatement, String> {
    assignment_statement(tokens)
}

fn assignment_statement(tokens: &mut VecDeque<Token>) -> Result<AstStatement, String> {
    reassignment_statement(tokens)
}

fn reassignment_statement(tokens: &mut VecDeque<Token>) -> Result<AstStatement, String> {
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
    fn test_assignment_statement() {
        let mut lexed = lexer::lex_from_string("let four = 4;".to_string()).unwrap();
        let statement = statement(&mut lexed).expect("Expected to return Ok");

        assert_eq!(statement, AstStatement::Assignment {
            name: "four".to_string(),
            expression: Box::new(NumberLiteral {
                value: "4".to_string(),
            })
        });
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
    }
}