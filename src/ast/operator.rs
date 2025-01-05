#[derive(PartialEq, Debug)]
pub enum OperatorType {
    Equality,
    Multiplication,
    Division,
    Modulus,
    Addition,
    Subtraction,
}

pub(super) fn parse(str: &str) -> Option<OperatorType> {
    Some(match str {
        "==" => OperatorType::Equality,
        
        "*" => OperatorType::Multiplication,
        "/" => OperatorType::Division,
        "%" => OperatorType::Modulus,
        
        "+" => OperatorType::Addition,
        "-" => OperatorType::Subtraction,
        
        _ => None?
    })
}

impl OperatorType {
    fn get_precedence(&self) -> u8 {
        match self {
            OperatorType::Equality => 2,

            OperatorType::Multiplication => 1,
            OperatorType::Division => 1,
            OperatorType::Modulus => 1,

            OperatorType::Addition => 0,
            OperatorType::Subtraction => 0
        }
    }
}

