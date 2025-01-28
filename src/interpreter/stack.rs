use crate::interpreter::value::RuntimeValue;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::option;

pub struct Stack {
    //parent: Option<Box<stack>>,
    frames: VecDeque<StackFrame>,
}

struct StackFrame {
    values: HashMap<String, RuntimeValue>,
}

impl Stack {
    pub fn new() -> Self {
        let mut frames = VecDeque::new();
        frames.push_back(StackFrame::new());
        Stack { frames }
    }

    pub fn get(&self, key: &str) -> Option<&RuntimeValue> {
        for frame in self.frames.iter().rev() {
            let value = frame.values.get(key);
            if value.is_some() { return value; }
        }
        None
    }

    pub fn declare(&mut self, key: String, value: RuntimeValue) -> Result<(), String> {
        let entry = self.frames.back_mut()
            .expect("No frames on the stack")
            .values.entry(key);

        match entry {
            Entry::Occupied(_) => { return Err(format!("Variable '{}' is already defined", entry.key())); }
            Entry::Vacant(_) => { entry.insert_entry(value); }
        }
        Ok(())
    }

    pub fn reassign(&mut self, key: &String, value: RuntimeValue) -> Result<(), String> {
        for frame in self.frames.iter_mut().rev() {
            if let Some(value_ref) = frame.values.get_mut(key) {
                *value_ref = value;
                return Ok(())
            }
        }

        Err(format!("Variable '{}' is not defined", key))
    }

    pub fn push_frame(&mut self) {
        self.frames.push_back(StackFrame::new());
    }

    pub fn pop_frame(&mut self) {
        self.frames.pop_front().expect("No frames on the stack");
    }
}

impl StackFrame {
    fn new() -> Self {
        StackFrame {
            values: HashMap::new()
        }
    }
}