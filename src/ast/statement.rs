use std::collections::VecDeque;
use crate::ast::AstStatement;
use crate::ast::AstStatement::FunctionCall;
use crate::ast::expression::expression;
use crate::ast::util::consume_control;
use crate::lexer::Token;
use crate::lexer::TokenType::{Control, Symbol};

pub(super) fn statement(tokens: &mut VecDeque<Token>) -> Result<AstStatement, String> {
    if let Some(first) = tokens.get(0) {
        if let Some(second) = tokens.get(1) {
            if first.token_type == Symbol && second.token_type == Control && second.contents == "(" {
                return function_call(tokens)
            }
        }
    }
    
    todo!()
}

fn function_call(tokens: &mut VecDeque<Token>) -> Result<AstStatement, String> {
    // Remove already matched tokens
    let name = tokens.pop_front().unwrap();
    tokens.pop_front().unwrap();
    
    // Currently only one argument is supported
    let argument = expression(tokens)?;
    
    consume_control(tokens, ")")?;
    consume_control(tokens, ";")?;
    
    Ok(FunctionCall {
        name: name.contents,
        arguments: vec![argument],
    })
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;
    use crate::ast::{AstExpression, AstStatement};
    use crate::ast::statement::statement;
    use crate::lexer::{Token, TokenType};

    #[test]
    fn test_function_call_parsing() {
        let mut tokens = VecDeque::from(vec![
            Token { token_type: TokenType::Symbol, contents: "println".to_string() },
            Token { token_type: TokenType::Control, contents: "(".to_string() },
            Token { token_type: TokenType::StringLiteral, contents: "\"Hello, world!\"".to_string() },
            Token { token_type: TokenType::Control, contents: ")".to_string() },
            Token { token_type: TokenType::Control, contents: ";".to_string() },
        ]);
        
        let statement = statement(&mut tokens).expect("Expected to return Ok");
        assert!(tokens.is_empty(), "Expected all tokens to have been consumed");
        assert_eq!(statement, AstStatement::FunctionCall { name: "println".to_string(), arguments: vec![
            AstExpression::StringLiteral { value: "Hello, world!".to_string() }
        ]});
    }
}