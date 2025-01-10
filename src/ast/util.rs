use std::collections::VecDeque;
use crate::ast::operator;
use crate::ast::operator::OperatorType;
use crate::lexer::{Token, TokenType};

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

pub(super) fn check_next<const N: usize>(tokens: &VecDeque<Token>, token_types: [TokenType; N]) -> bool {
    if tokens.is_empty() { return false; }
    if let Some(token) = tokens.get(0) {
        token_types.iter().any(|t| token.token_type == *t)
    } else {
        false
    }
}