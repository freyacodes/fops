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
