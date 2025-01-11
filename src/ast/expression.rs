use crate::ast::operator::OperatorType::{Plus, Division, Equality, Modulus, Multiplication, Minus, Bang, Inequality};
use crate::ast::AstElement::{BooleanLiteral, NumberLiteral, StringLiteral, Symbol, UnaryOperator};
use crate::ast::{util, AstElement};
use crate::lexer::{Token, TokenType};
use std::collections::VecDeque;

pub fn parse(mut tokens: VecDeque<Token>) -> Result<AstElement, String> {
    expression(&mut tokens)
}

fn expression(tokens: &mut VecDeque<Token>) -> Result<AstElement, String> {
    equality(tokens)
}

fn equality(tokens: &mut VecDeque<Token>) -> Result<AstElement, String> {
    let mut expression = comparison(tokens)?;

    loop {
        if let Some(operator) = util::match_operator(tokens, [Equality, Inequality]) {
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
        if let Some(operator) = util::match_operator(tokens, [Plus, Minus]) {
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
    if let Some(operator) = util::match_operator(tokens, [Minus, Bang]) {
        return Ok(UnaryOperator {
            operator,
            operand: Box::new(unary(tokens)?),
        });
    }

    primary(tokens)
}

fn primary(tokens: &mut VecDeque<Token>) -> Result<AstElement, String> {
    if let Some(next_token) = tokens.pop_front() {
        if next_token.token_type == TokenType::Number {
            return Ok(NumberLiteral { value: next_token.contents });
        }
        if next_token.token_type == TokenType::StringLiteral {
            let len = next_token.contents.len();
            return Ok(StringLiteral { 
                // Remove the quotes
                value: next_token.contents.chars().skip(1).take(len-2).collect() 
            });
        }
        if next_token.token_type == TokenType::Symbol {
            return match next_token.contents.as_str() {
                "true" => Ok(BooleanLiteral { value: true }),
                "false" => Ok(BooleanLiteral { value: false }),
                _ => Ok(Symbol { name: next_token.contents })
            };
        }
        if next_token.token_type == TokenType::Special && next_token.contents == "(" {
            let expression = expression(tokens)?;
            if !util::match_special(tokens, ")") {
                return Err("Expected parenthesis close ')'".to_string());
            }
            return Ok(expression);
        }

        return Err(format!("Unexpected end of expression {}", next_token.contents))
    }
    Err("Unexpected end of expression".to_string())
}

#[cfg(test)]
mod test {
    use crate::ast::expression;
    use crate::ast::operator::OperatorType;
    use crate::ast::operator::OperatorType::{Division, Multiplication};
    use crate::ast::AstElement::{BiOperator, NumberLiteral, Symbol, UnaryOperator};
    use crate::lexer;
    use crate::lexer::Token;
    use std::collections::VecDeque;

    #[test]
    fn parenthesis_division_test() {
        let lexed = lexer::lex_from_string("(-500*bar)/10".to_string()).unwrap().into_iter().flatten().collect::<VecDeque<Token>>();
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

        assert_eq!(expression::parse(VecDeque::from(lexed)), Ok(expected));
    }
}