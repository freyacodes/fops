use crate::interpreter::function::FunctionImplementation;

#[derive(Debug, PartialEq, Clone)]
pub enum RuntimeValue {
    String(String),
    Integer(i32),
    Boolean(bool),
    Function { arity: u8, implementation: FunctionImplementation },
    Unit
}

impl RuntimeValue {
    pub fn value_as_string(&self) -> String {
        match self {
            RuntimeValue::String(string) => string.to_string(),
            RuntimeValue::Integer(int) => int.to_string(),
            RuntimeValue::Boolean(bool) => bool.to_string(),
            RuntimeValue::Function { .. } => { "<function>".to_string() }
            RuntimeValue::Unit => "unit".to_string()
        }
    }

    pub fn type_name(&self) -> &str {
        match self {
            RuntimeValue::String(_) => "string",
            RuntimeValue::Integer(_) => "integer",
            RuntimeValue::Boolean(_) => "boolean",
            RuntimeValue::Function { .. } => "function",
            RuntimeValue::Unit => "unit"
        }
    }
}
