use std::collections::VecDeque;
use crate::ast::AstStatement;
use crate::ast::expression::expression;
use crate::ast::util::consume_control;
use crate::lexer::Token;
use crate::lexer::TokenType::{Control, Symbol};

pub(super) fn statement(tokens: &mut VecDeque<Token>) -> Result<AstStatement, String> {
    todo!()
}