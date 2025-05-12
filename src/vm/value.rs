use std::fmt::{Display, Formatter};

pub const NIL: Value = Value::Nil;
pub const TRUE: Value = Value::Bool(true);
pub const FALSE: Value = Value::Bool(false);

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Nil,
    Obj(Obj)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Obj {
    StringObj { value: String }
}

impl Value {
    pub fn is_string(&self) -> bool {
        match self { 
            Value::Obj(Obj::StringObj { .. }) => true,
            _ => false
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(number) => write!(f, "{}", number),
            Value::Bool(bool) => write!(f, "{}", bool),
            Value::Nil => write!(f, "nil"),
            Value::Obj(obj) => write!(f, "{}", obj),
        }
    }
}

impl Display for Obj {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Obj::StringObj { value } => write!(f, "{}", value),
        }
    }
}

impl From<String> for Value {
    fn from(string: String) -> Value {
        Value::Obj(Obj::StringObj { value: string })
    }
}

impl From<&str> for Value {
    fn from(string: &str) -> Value {
        Value::from(string.to_string())
    }
}