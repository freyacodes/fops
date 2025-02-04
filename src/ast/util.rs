use crate::ast::operator;
use crate::ast::operator::OperatorType;
use crate::lexer::TokenType::Keyword;
use crate::lexer::{Token, TokenType};
use std::collections::VecDeque;
use TokenType::Control;

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

pub(super) fn match_control(tokens: &mut VecDeque<Token>, expected: &str) -> bool {
    match_type_and_content(tokens, Control, expected)
}

pub(super) fn match_keyword(tokens: &mut VecDeque<Token>, expected: &str) -> bool {
    match_type_and_content(tokens, Keyword, expected)
}

fn match_type_and_content(tokens: &mut VecDeque<Token>, token_type: TokenType, expected: &str) -> bool {
    if let Some(token) = tokens.get(0) {
        if token.token_type == token_type && token.contents == expected {
            tokens.pop_front();
            return true
        }
    }
    false
}

pub(super) fn consume_control(tokens: &mut VecDeque<Token>, expected: &str) -> Result<(), String> {
    if let Some(token) = tokens.pop_front() {
        if token.token_type == Control && token.contents == expected {
            Ok(())
        } else {
            Err(format!("Expected '{}', encountered {}", expected, token.contents))
        }
    } else {
        Err(format!("Expected '{}', encountered end of input", expected))
    }
}
