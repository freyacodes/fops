use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Nil
}

impl Display for Value {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(number) => write!(f, "{}", number),
            Value::Bool(bool) => write!(f, "{}", bool),
            Value::Nil => write!(f, "nil"),
        }
    }
}


