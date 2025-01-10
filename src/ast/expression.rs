use crate::ast::operator::OperatorType::{Addition, Division, Equality, Modulus, Multiplication, Subtraction};
use crate::ast::AstElement::{NumberLiteral, UnaryOperator};
use crate::ast::{util, AstElement};
use crate::lexer::{Token, TokenType};
use std::collections::VecDeque;

pub(super) fn parse(mut tokens: VecDeque<Token>) -> Result<AstElement, String> {
    expression(&mut tokens)
}

fn expression(tokens: &mut VecDeque<Token>) -> Result<AstElement, String> {
    equality(tokens)
}

fn equality(tokens: &mut VecDeque<Token>) -> Result<AstElement, String> {
    let mut expression = comparison(tokens)?;

    loop {
        if let Some(operator) = util::match_operator(tokens, [Equality]) {
            expression = AstElement::BiOperator {
                operator,
                left: Box::new(expression),
                right: Box::new(comparison(tokens)?),
            }
        } else { break; }
    }

    Ok(expression)
}

fn comparison(tokens: &mut VecDeque<Token>) -> Result<AstElement, String> {
    // TODO: Add comparison to the language
    term(tokens)
}

fn term(tokens: &mut VecDeque<Token>) -> Result<AstElement, String> {
    let mut expression = factor(tokens)?;

    loop {
        if let Some(operator) = util::match_operator(tokens, [Addition, Subtraction]) {
            expression = AstElement::BiOperator {
                operator,
                left: Box::new(expression),
                right: Box::new(factor(tokens)?),
            }
        } else { break; }
    }

    Ok(expression)
}

fn factor(tokens: &mut VecDeque<Token>) -> Result<AstElement, String> {
    let mut expression = unary(tokens)?;

    loop {
        if let Some(operator) = util::match_operator(tokens, [Multiplication, Division, Modulus]) {
            expression = AstElement::BiOperator {
                operator,
                left: Box::new(expression),
                right: Box::new(unary(tokens)?),
            }
        } else { break; }
    }

    Ok(expression)
}

fn unary(tokens: &mut VecDeque<Token>) -> Result<AstElement, String> {
    if let Some(operator) = util::match_operator(tokens, [Subtraction]) {
        return Ok(UnaryOperator {
            operator,
            operand: Box::new(unary(tokens)?)
        });
    }

    primary(tokens)
}

fn primary(tokens: &mut VecDeque<Token>) -> Result<AstElement, String> {
    if let Some(next_token) = tokens.pop_front() {
        if next_token.token_type == TokenType::Number {
            return Ok(NumberLiteral { value: next_token.contents })
        }
        if next_token.token_type == TokenType::Symbol {
            return Ok(NumberLiteral { value: next_token.contents })
        }
        if next_token.token_type == TokenType::Symbol && next_token.contents == "(" {
            let expression = expression(tokens)?;
            if !util::match_special(tokens, ")") { 
                return Err("Expected parenthesis close ')'".to_string())
            }
            return Ok(expression)
        }
    }
    Err("Unexpected end of expression".to_string())
}