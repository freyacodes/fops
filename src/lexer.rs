use std::fs;
use std::path::Path;

const SPECIAL_CHARACTERS: [char; 8] = ['+', '-', '*', '/', '=', '{', '}', ';'];
const MULTICHAR_OPERATORS: [char; 1] = ['='];

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TokenType {
    None,
    Symbol,
    Number,
    Special,
    StringLiteral,
}

#[derive(PartialEq, Debug)]
pub struct Token {
    token_type: TokenType,
    contents: String,
}

pub fn lex_from_file(file: Box<Path>) -> Result<Vec<Vec<Token>>, String> {
    let file_name = file.file_name()
        .expect("File name does not end in ..")
        .to_str()
        .expect("Unable to read file name")
        .to_string();

    let file_contents = match fs::read_to_string(file) {
        Ok(contents) => contents,
        Err(err) => return Err(format!("Unable to read file {}: {}", file_name, err))
    };

    lex_from_string(file_contents)
}

pub fn lex_from_string(string: String) -> Result<Vec<Vec<Token>>, String> {
    let mut tokenized_lines: Vec<Vec<Token>> = Vec::new();
    for (line_index, line) in string.lines().enumerate() {
        tokenized_lines.push(lex_line(line, line_index)?);
    }

    Ok(tokenized_lines)
}

fn lex_line(line: &str, line_index: usize) -> Result<Vec<Token>, String> {
    let mut buffer: Vec<char> = Vec::new();
    let mut tokens: Vec<Token> = Vec::new();
    let mut token_type = TokenType::None;

    fn terminate_token(buffer: &mut Vec<char>, tokens: &mut Vec<Token>, token_type: &mut TokenType) {
        if buffer.is_empty() { return; }
        tokens.push(Token {
            token_type: *token_type,
            contents: buffer.drain(0..).collect::<String>()
        });
        *token_type = TokenType::None;
    }

    for (col, c) in line.chars().enumerate() {
        if token_type == TokenType::StringLiteral {
            if c == '"' {
                buffer.push(c);
                terminate_token(&mut buffer, &mut tokens, &mut token_type);
            } else {
                buffer.push(c);
            }
            continue;
        } else if c.is_whitespace() {
            terminate_token(&mut buffer, &mut tokens, &mut token_type);
            continue;
        }

        let is_special_after_non_special = token_type != TokenType::Special && SPECIAL_CHARACTERS.contains(&c);
        if is_special_after_non_special {
            // Terminate the last token, and proceed with handling this special character
            terminate_token(&mut buffer, &mut tokens, &mut token_type);
        }
        
        let is_new_token_after_special = token_type == TokenType::Special && !SPECIAL_CHARACTERS.contains(&c);
        if  token_type == TokenType::None || is_new_token_after_special {
            terminate_token(&mut buffer, &mut tokens, &mut token_type);
            buffer.push(c);
            if c.is_ascii_alphabetic() || c == '_' {
                token_type = TokenType::Symbol;
            } else if c.is_ascii_digit() {
                token_type = TokenType::Number;
            } else if c == '"' {
                token_type = TokenType::StringLiteral;
            } else if SPECIAL_CHARACTERS.contains(&c) {
                token_type = TokenType::Special;
                if !MULTICHAR_OPERATORS.contains(&c) {
                    terminate_token(&mut buffer, &mut tokens, &mut token_type);
                }
            }
            continue
        }

        match token_type {
            TokenType::Symbol => {
                if !c.is_ascii_alphanumeric() {
                    return Err(format!("Unexpected character '{}' at {}:{}", c, line_index + 1, col + 1));
                }
                buffer.push(c);
            }
            TokenType::Number => {
                if !c.is_ascii_digit() {
                    return Err(format!("Unexpected character '{}' at {}:{}", c, line_index + 1, col + 1));
                }
                buffer.push(c);
            }
            TokenType::Special => {
                if !SPECIAL_CHARACTERS.contains(&c) {
                    return Err(format!("Unexpected character '{}' at {}:{}", c, line_index + 1, col + 1));
                }
                buffer.push(c);
            }
            TokenType::StringLiteral => unreachable!(),
            TokenType::None => unreachable!()
        }
    }

    terminate_token(&mut buffer, &mut tokens, &mut token_type);
    Ok(tokens)
}

#[cfg(test)]
mod test {
    use crate::lexer::{lex_line, Token};
    use crate::lexer::TokenType::{Number, Special, StringLiteral, Symbol};

    #[test]
    fn test_string_assignment() {
        let line = "let foo=\"bar\"";
        let expected = vec![
            Token { token_type: Symbol, contents: "let".to_string() },
            Token { token_type: Symbol, contents: "foo".to_string() },
            Token { token_type: Special, contents: "=".to_string() },
            Token { token_type: StringLiteral, contents: "\"bar\"".to_string() },
            Token { token_type: Special, contents: ";".to_string() },
        ];
        
        assert_eq!(lex_line(&String::from(line), 0).unwrap(), expected);
    }

    #[test]
    fn test_arithmetic_assignment() {
        let line = "let foo = (-500*bar)/10;";
        let expected = vec![
            Token { token_type: Symbol, contents: "let".to_string() },
            Token { token_type: Symbol, contents: "foo".to_string() },
            Token { token_type: Special, contents: "=".to_string() },
            Token { token_type: Special, contents: "(".to_string() },
            Token { token_type: Special, contents: "-".to_string() },
            Token { token_type: Number, contents: "500".to_string() },
            Token { token_type: Special, contents: "*".to_string() },
            Token { token_type: Symbol, contents: "bar".to_string() },
            Token { token_type: Special, contents: ")".to_string() },
            Token { token_type: Special, contents: "/".to_string() },
            Token { token_type: Number, contents: "10".to_string() },
            Token { token_type: Special, contents: ";".to_string() },
        ];

        assert_eq!(lex_line(&String::from(line), 0).unwrap(), expected);
    }

    #[test]
    fn test_equality() {
        let line = "if foo==500{";
        let expected = vec![
            Token { token_type: Symbol, contents: "if".to_string() },
            Token { token_type: Symbol, contents: "foo".to_string() },
            Token { token_type: Special, contents: "==".to_string() },
            Token { token_type: Number, contents: "500".to_string() },
            Token { token_type: Special, contents: "{".to_string() },
        ];

        assert_eq!(lex_line(&String::from(line), 0).unwrap(), expected);
    }
}