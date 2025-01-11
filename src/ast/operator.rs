use std::fmt;

#[derive(PartialEq, Debug)]
pub enum OperatorType {
    Equality,
    Inequality,
    Bang,
    Multiplication,
    Division,
    Modulus,
    Plus,
    Minus,
}

pub(super) fn parse(str: &str) -> Option<OperatorType> {
    Some(match str {
        "==" => OperatorType::Equality,
        "!=" => OperatorType::Inequality,
        "!" => OperatorType::Bang,

        "*" => OperatorType::Multiplication,
        "/" => OperatorType::Division,
        "%" => OperatorType::Modulus,

        "+" => OperatorType::Plus,
        "-" => OperatorType::Minus,

        _ => None?
    })
}

impl fmt::Display for OperatorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}