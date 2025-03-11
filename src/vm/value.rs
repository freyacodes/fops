pub enum Value {
    Number(f64),
    Bool(bool),
    Nil
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Number(number) => number.to_string(),
            Value::Bool(bool) => bool.to_string(),
            Value::Nil => "nil".to_string()
        }
    }
}


