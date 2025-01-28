use crate::interpreter::value::RuntimeValue;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub(super) struct Stack {
    frames: VecDeque<StackFrame>,
}

#[derive(Debug)]
struct StackFrame {
    values: HashMap<String, RuntimeValue>,
}

impl Stack {
    pub fn new(globals: HashMap<String, RuntimeValue>) -> Self {
        let mut frames = VecDeque::new();
        frames.push_back(StackFrame::new(globals));
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
                println!("{:?}", self);
                return Ok(())
            }
        }

        Err(format!("Variable '{}' is not defined", key))
    }

    pub fn push_frame(&mut self) {
        self.frames.push_back(StackFrame::new_empty());
    }

    pub fn pop_frame(&mut self) {
        if self.frames.len() == 1 {
            panic!("Cannot pop the last frame");
        }
        self.frames.pop_back().unwrap();
    }

    /// Returns the top-level variables
    pub fn dismantle(mut self) -> HashMap<String, RuntimeValue> {
        self.frames.pop_front().unwrap().values
    }
}

impl StackFrame {
    fn new_empty() -> Self {
        StackFrame {
            values: HashMap::new()
        }
    }

    fn new(values: HashMap<String, RuntimeValue>) -> Self {
        StackFrame {
            values
        }
    }
}