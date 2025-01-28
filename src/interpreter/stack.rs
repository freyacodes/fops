use crate::interpreter::value::RuntimeValue;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct Stack {
    //parent: Option<Box<stack>>,
    values: HashMap<String, RuntimeValue>
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            values: HashMap::new()
        }
    }

    pub fn get(&self, key: &str) -> Option<&RuntimeValue> {
        self.values.get(key)
    }

    pub fn declare(&mut self, key: String, value: RuntimeValue) -> Result<(), String> {
        let entry = self.values.entry(key);
        match entry {
            Entry::Occupied(_) => { return Err(format!("Variable '{}' is already defined", entry.key())); },
            Entry::Vacant(_) => { entry.insert_entry(value); }
        }
        Ok(())
    }

    pub fn reassign(&mut self, key: &String, value: RuntimeValue) -> Result<(), String> {
        let option = self.values.get_mut(key);
        match option {
            Some(v) => { *v = value },
            None => { return Err(format!("Variable '{}' is not found", key)); }
        }
        Ok(())
    }
}