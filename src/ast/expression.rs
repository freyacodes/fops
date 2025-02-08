use crate::ast::operator::OperatorType::{Bang, Division, Equality, GreaterThan, GreaterThanOrEqual, Inequality, LessThan, LessThanOrEqual, Minus, Modulus, Multiplication, Plus};
use crate::ast::util::match_control;
use crate::ast::AstExpression::{BiOperator, BooleanLiteral, Logical, NumberLiteral, StringLiteral, Symbol, UnaryOperator};
use crate::ast::LogicalOperator::{And, Or};
use crate::ast::{util, AstExpression};
use crate::lexer::{Token, TokenType};
use std::collections::VecDeque;

pub(super) fn expression(tokens: &mut VecDeque<Token>) -> Result<AstExpression, String> {
    logic_or(tokens)
}

fn logic_or(tokens: &mut VecDeque<Token>) -> Result<AstExpression, String> {
    let mut expression = logic_and(tokens)?;

    while match_control(tokens, "||") {
        let right = logic_and(tokens)?;
        expression = Logical {
            operator: Or,
            left: Box::new(expression),
            right: Box::new(right),
        }
    }

    Ok(expression)
}

fn logic_and(tokens: &mut VecDeque<Token>) -> Result<AstExpression, String> {
    let mut expression = equality(tokens)?;

    while match_control(tokens, "&&") {
        let right = equality(tokens)?;
        expression = Logical {
            operator: And,
            left: Box::new(expression),
            right: Box::new(right),
        }
    }

    Ok(expression)
}

fn equality(tokens: &mut VecDeque<Token>) -> Result<AstExpression, String> {
    let mut expression = comparison(tokens)?;

    loop {
        if let Some(operator) = util::match_operator(tokens, [Equality, Inequality]) {
            expression = BiOperator {
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
            expression = BiOperator {
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
            expression = BiOperator {
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
            expression = BiOperator {
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

    call(tokens)
}

fn call(tokens: &mut VecDeque<Token>) -> Result<AstExpression, String> {
    let primary = primary(tokens)?;
    
    if !match_control(tokens, "(") { return Ok(primary) }

    let mut arguments: Vec<AstExpression> = Vec::new();
    loop {
        if match_control(tokens, ")") {
            return Ok(AstExpression::Call { callee: Box::new(primary), arguments })
        }
        arguments.push(expression(tokens)?);
    }
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
            if !match_control(tokens, ")") {
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
    use crate::ast::AstExpression::{BiOperator, Call, Logical, NumberLiteral, StringLiteral, Symbol, UnaryOperator};
    use crate::ast::LogicalOperator::{And, Or};
    use crate::{ast, lexer};
    use std::collections::VecDeque;

    #[test]
    fn test_call_parsing_zero_args() {
        let mut lexed = lexer::lex_from_string("panic()".to_string()).unwrap();

        let expression = expression(&mut lexed).expect("Expected to return Ok");
        assert_eq!(expression, Call {
            callee: Box::new(Symbol { name: "panic".to_string() }),
            arguments: vec![]
        });
    }

    #[test]
    fn test_call_parsing_one_arg() {
        let mut lexed = lexer::lex_from_string("println(\"Hello, world!\")".to_string()).unwrap();

        let expression = expression(&mut lexed).expect("Expected to return Ok");
        assert_eq!(expression, Call {
            callee: Box::new(Symbol { name: "println".to_string() }),
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

    #[test]
    fn test_logical_expressions() {
        let lexed = lexer::lex_from_string("a && b || c".to_string()).unwrap();
        let expected = Logical {
            operator: Or,
            left: Box::new(Logical {
                operator: And,
                left: Box::new(Symbol { name: "a".to_string() }),
                right: Box::new(Symbol { name: "b".to_string() }),
            }),
            right: Box::new(Symbol { name: "c".to_string() }),
        };
        assert_eq!(ast::parse_expression_only(VecDeque::from(lexed)), Ok(expected));
    }
}
