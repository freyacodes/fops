pub mod operator;
mod expression;
mod util;
mod statement;

use crate::lexer::Token;
use operator::OperatorType;
use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
pub enum AstStatement {
    If { conditional_clauses: Vec<ConditionalClause>, else_clause: Option<Box<AstStatement>> },
    While { condition: AstExpression, statement: Box<AstStatement> },
    Block { statements: Vec<AstStatement> },
    Declaration { name: String, expression: AstExpression },
    Reassignment { name: String, expression: AstExpression },
    Expression { expression: AstExpression }
}

#[derive(PartialEq, Debug)]
pub struct ConditionalClause {
    pub condition: AstExpression,
    pub statement: AstStatement
}

#[derive(PartialEq, Debug)]
pub enum AstExpression {
    Logical { operator: LogicalOperator, left: Box<AstExpression>, right: Box<AstExpression> },
    BiOperator { operator: OperatorType, left: Box<AstExpression>, right: Box<AstExpression> },
    UnaryOperator { operator: OperatorType, operand: Box<AstExpression> },
    NumberLiteral { value: String },
    StringLiteral { value: String },
    BooleanLiteral { value: bool },
    Symbol { name: String },
    Call { callee: Box<AstExpression>, arguments: Vec<AstExpression> }
}

#[derive(PartialEq, Debug)]
pub enum LogicalOperator { And, Or }

pub fn parse_script(mut tokens: VecDeque<Token>) -> Result<Vec<AstStatement>, String> {
    let mut statements: Vec<AstStatement> = Vec::new();

    while !tokens.is_empty() {
        statements.push(statement::statement(&mut tokens)?)
    }
    
    Ok(statements)
}

pub fn parse_expression_only(mut tokens: VecDeque<Token>) -> Result<AstExpression, String> {
    expression::expression(&mut tokens)
}
