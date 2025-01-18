#[derive(Debug, PartialEq, Clone)]
pub enum RuntimeValue {
    String(String),
    Integer(i32),
    Boolean(bool),
    Unit
}

impl RuntimeValue {
    pub fn value_as_string(&self) -> String {
        match self {
            RuntimeValue::String(string) => string.to_string(),
            RuntimeValue::Integer(int) => int.to_string(),
            RuntimeValue::Boolean(bool) => bool.to_string(),
            RuntimeValue::Unit => "unit".to_string()
        }
    }
}