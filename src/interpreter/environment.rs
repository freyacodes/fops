use std::any::Any;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use crate::interpreter::value::RuntimeValue;

struct Environment {
    //parent: Option<Box<Environment>>,
    values: HashMap<String, RuntimeValue>
}

impl Environment {
    fn new() -> Self {
        Environment {
            values: HashMap::new()
        }
    }
    
    fn get(&self, key: &str) -> Option<&RuntimeValue> {
        self.values.get(key)
    }
    
    fn declare(&mut self, key: String, value: RuntimeValue) -> Result<(), String> {
        let entry = self.values.entry(key);
        match entry {
            Entry::Occupied(_) => { return Err(format!("Variable '{}' is already defined", entry.key())); },
            Entry::Vacant(_) => { entry.insert_entry(value); }
        }
        Ok(())
    }

    fn reassign(&mut self, key: String, value: RuntimeValue) -> Result<(), String> {
        let entry = self.values.entry(key);
        match entry {
            Entry::Occupied(_) => { entry.insert_entry(value); },
            Entry::Vacant(_) => { return Err(format!("Variable '{}' is not found", entry.key())); }
        }
        Ok(())
    }
}