use crate::ast::expression::expression;
use crate::ast::util::consume_control;
use crate::ast::AstStatement;
use crate::lexer::Token;
use std::collections::VecDeque;

pub(super) fn statement(tokens: &mut VecDeque<Token>) -> Result<AstStatement, String> {
    expression_statement(tokens)
}

fn expression_statement(tokens: &mut VecDeque<Token>) -> Result<AstStatement, String> {
    let expression = expression(tokens)?;
    consume_control(tokens, ";")?;
    Ok(AstStatement::Expression { expression: Box::new(expression) })
}

#[cfg(test)]
mod test {
    use crate::ast::AstExpression::{FunctionCall, StringLiteral};
    use crate::ast::expression::expression;
    use crate::lexer;

    #[test]
    fn test_expression_statement() {
        let mut lexed = lexer::lex_from_string("println(\"Hello, world!\");".to_string()).unwrap();

        let expression = expression(&mut lexed).expect("Expected to return Ok");
        assert_eq!(expression, FunctionCall {
            name: "println".to_string(),
            arguments: vec![
                StringLiteral { value: "Hello, world!".to_string() }
            ]
        });
    }
}