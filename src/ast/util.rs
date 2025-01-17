use crate::ast::operator;
use crate::ast::operator::OperatorType;
use crate::lexer::{Token, TokenType};
use std::collections::VecDeque;

pub(super) fn match_operator<const N: usize>(tokens: &mut VecDeque<Token>, operators: [OperatorType; N]) -> Option<OperatorType> {
    if let Some(token) = tokens.get(0) {
        if let Some(operator) = operator::parse(&*token.contents) {
            if operators.iter().any(|o| o == &operator) {
                tokens.pop_front();
                return Some(operator)
            }
        }
    }
    None
}

pub(super) fn match_special(tokens: &mut VecDeque<Token>, expected: &str) -> bool {
    if let Some(token) = tokens.get(0) {
        if token.contents == expected {
            tokens.pop_front();
            return true
        }
    }
    false
}

pub(super) fn consume_control(tokens: &mut VecDeque<Token>, expected: &str) -> Result<(), String> {
    if let Some(token) = tokens.pop_front() {
        if token.token_type == TokenType::Control && token.contents == expected {
            Ok(())
        } else {
            Err(format!("Expected '{}', encountered {}", expected, token.contents))
        }
    } else {
        Err(format!("Expected '{}', encountered end of input", expected))
    }
}
