use crate::ast::{operator, AstElement};
use crate::ast::AstElement::{NumberLiteral, Symbol};
use crate::ast::operator::OperatorType;
use crate::lexer::{Token, TokenType};

pub(super) fn parse(tokens: Vec<Token>) -> Result<AstElement, String> {
    let mut tokens_with_operators: Vec<(Token, Option<OperatorType>)> = Vec::new();

    for token in tokens {
        let tuple = match token.token_type {
            TokenType::Special => {
                let operator_opt = operator::parse(token.contents.as_str());
                (token, operator_opt)
            },
            TokenType::Symbol | TokenType::Number => (token, None),
            _ => Err(format!("Unexpected {} when parsing statement", token.contents).to_string())?
        };
        tokens_with_operators.push(tuple);
    }

    parse_statement_recursive(tokens_with_operators)
}

fn parse_statement_recursive(mut tokens: Vec<(Token, Option<OperatorType>)>) -> Result<AstElement, String> {
    if tokens.len() == 1 {
        let (last_token, _) = tokens.pop().unwrap();
        return Ok(match last_token.token_type {
            TokenType::Number => { NumberLiteral { value: last_token.contents } },
            TokenType::Symbol => { Symbol { name: last_token.contents } }
            _ => return Err(format!("Unexpected token '{}' in statement", last_token.contents))
        })
    }

    todo!()
}