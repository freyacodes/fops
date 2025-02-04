use crate::ast::operator::OperatorType::{Bang, Division, Equality, GreaterThan, GreaterThanOrEqual, Inequality, LessThan, LessThanOrEqual, Minus, Modulus, Multiplication, Plus};
use crate::ast::util::consume_control;
use crate::ast::AstExpression::{BooleanLiteral, NumberLiteral, StringLiteral, Symbol, UnaryOperator};
use crate::ast::{util, AstExpression};
use crate::lexer::{Token, TokenType};
use std::collections::VecDeque;

pub(super) fn expression(tokens: &mut VecDeque<Token>) -> Result<AstExpression, String> {
    equality(tokens)
}

fn equality(tokens: &mut VecDeque<Token>) -> Result<AstExpression, String> {
    let mut expression = comparison(tokens)?;

    loop {
        if let Some(operator) = util::match_operator(tokens, [Equality, Inequality]) {
            expression = AstExpression::BiOperator {
                operator,
                left: Box::new(expression),
                right: Box::new(comparison(tokens)?),
            }
        } else { break; }
    }

    Ok(expression)
}

fn comparison(tokens: &mut VecDeque<Token>) -> Result<AstExpression, String> {
    let mut expression = term(tokens)?;

    loop {
        if let Some(operator) = util::match_operator(tokens, [LessThan, GreaterThan, LessThanOrEqual, GreaterThanOrEqual]) {
            expression = AstExpression::BiOperator {
                operator,
                left: Box::new(expression),
                right: Box::new(factor(tokens)?),
            }
        } else { break; }
    }

    Ok(expression)
}

fn term(tokens: &mut VecDeque<Token>) -> Result<AstExpression, String> {
    let mut expression = factor(tokens)?;

    loop {
        if let Some(operator) = util::match_operator(tokens, [Plus, Minus]) {
            expression = AstExpression::BiOperator {
                operator,
                left: Box::new(expression),
                right: Box::new(factor(tokens)?),
            }
        } else { break; }
    }

    Ok(expression)
}

fn factor(tokens: &mut VecDeque<Token>) -> Result<AstExpression, String> {
    let mut expression = unary(tokens)?;

    loop {
        if let Some(operator) = util::match_operator(tokens, [Multiplication, Division, Modulus]) {
            expression = AstExpression::BiOperator {
                operator,
                left: Box::new(expression),
                right: Box::new(unary(tokens)?),
            }
        } else { break; }
    }

    Ok(expression)
}

fn unary(tokens: &mut VecDeque<Token>) -> Result<AstExpression, String> {
    if let Some(operator) = util::match_operator(tokens, [Minus, Bang]) {
        return Ok(UnaryOperator {
            operator,
            operand: Box::new(unary(tokens)?),
        });
    }

    if let Some(first) = tokens.get(0) {
        if let Some(second) = tokens.get(1) {
            if first.token_type == TokenType::Symbol
                && second.token_type == TokenType::Control
                && second.contents == "(" {
                return call(tokens)
            }
        }
    }

    primary(tokens)
}

fn call(tokens: &mut VecDeque<Token>) -> Result<AstExpression, String> {
    // Remove already matched tokens
    let name = tokens.pop_front().unwrap();
    tokens.pop_front().unwrap();

    // Currently only one argument is supported
    let argument = expression(tokens)?;

    consume_control(tokens, ")")?;

    Ok(AstExpression::FunctionCall {
        name: name.contents,
        arguments: vec![argument],
    })
}

fn primary(tokens: &mut VecDeque<Token>) -> Result<AstExpression, String> {
    if let Some(next_token) = tokens.pop_front() {
        if next_token.token_type == TokenType::Number {
            return Ok(NumberLiteral { value: next_token.contents });
        }
        if next_token.token_type == TokenType::StringLiteral {
            let len = next_token.contents.len();
            return Ok(StringLiteral {
                // Remove the quotes
                value: next_token.contents.chars().skip(1).take(len - 2).collect()
            });
        }
        if next_token.token_type == TokenType::Symbol {
            return match next_token.contents.as_str() {
                "true" => Ok(BooleanLiteral { value: true }),
                "false" => Ok(BooleanLiteral { value: false }),
                _ => Ok(Symbol { name: next_token.contents })
            };
        }
        if next_token.token_type == TokenType::Control && next_token.contents == "(" {
            let expression = expression(tokens)?;
            if !util::match_control(tokens, ")") {
                return Err("Expected parenthesis close ')'".to_string());
            }
            return Ok(expression);
        }

        return Err(format!("Unexpected end of expression {}", next_token.contents));
    }
    Err("Unexpected end of expression".to_string())
}

#[cfg(test)]
mod test {
    use crate::ast::expression::expression;
    use crate::ast::operator::OperatorType;
    use crate::ast::operator::OperatorType::{Division, Multiplication};
    use crate::ast::AstExpression::{BiOperator, FunctionCall, NumberLiteral, StringLiteral, Symbol, UnaryOperator};
    use crate::{ast, lexer};
    use std::collections::VecDeque;

    #[test]
    fn test_function_call_parsing() {
        let mut lexed = lexer::lex_from_string("println(\"Hello, world!\")".to_string()).unwrap();

        let expression = expression(&mut lexed).expect("Expected to return Ok");
        assert_eq!(expression, FunctionCall {
            name: "println".to_string(),
            arguments: vec![
                StringLiteral { value: "Hello, world!".to_string() }
            ]
        });
    }

    #[test]
    fn test_parenthesis_division() {
        let lexed = lexer::lex_from_string("(-500*bar)/10".to_string()).unwrap();
        let expected = BiOperator {
            operator: Division,
            left: Box::new(BiOperator {
                operator: Multiplication,
                left: Box::new(UnaryOperator {
                    operator: OperatorType::Minus,
                    operand: Box::new(NumberLiteral { value: "500".to_string() }),
                }),
                right: Box::new(Symbol { name: "bar".to_string() }),
            }),
            right: Box::new(NumberLiteral { value: "10".to_string() }),
        };

        assert_eq!(ast::parse_expression_only(VecDeque::from(lexed)), Ok(expected));
    }
}
