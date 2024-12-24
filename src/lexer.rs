use std::fs;
use std::path::Path;

const SPECIAL_CHARACTERS: [char; 8] = ['+', '-', '*', '/', '=', '{', '}', ';'];

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

pub fn lex(file: Box<Path>) -> Result<Vec<Vec<Token>>, String> {
    let file_name = file.file_name()
        .expect("File name does not end in ..")
        .to_str()
        .expect("Unable to read file name")
        .to_string();

    let file_contents = match fs::read_to_string(file) {
        Ok(contents) => contents,
        Err(err) => return Err(format!("Unable to read file {}: {}", file_name, err))
    };
    
    let mut tokenized_lines: Vec<Vec<Token>> = Vec::new();
    for (line_index, line) in file_contents.lines().enumerate() {
        tokenized_lines.push(lex_line(line, line_index)?);
    }

    Ok(tokenized_lines)
}

fn lex_line(line: &str, line_index: usize) -> Result<Vec<Token>, String> {

}

#[cfg(test)]
mod test {
    use crate::lexer::{lex_line, Token};
    use crate::lexer::TokenType::{Number, Special, StringLiteral, Symbol};

    #[test]
    fn test_string_assignment() {
        let line = "let foo = \"bar\"";
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
        let line = "if foo == 500 {";
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