use crate::bytecode::codes::{OP_CONTANT, OP_F64};
use crate::vm::value::Value;

pub struct Chunk {
    pub(crate) code: Vec<u8>,
    constants: Vec<Value>,
    lines: Vec<u16>
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(), 
            constants: Vec::new(),
            lines: Vec::new()
        }
    }

    pub fn write(&mut self, op: u8, line: u16) {
        self.code.push(op);
        self.lines.push(line);
    }

    pub fn write0(&mut self, op: u8) {
        self.write(op, 0);
    }

    pub fn write_f64(&mut self, float: f64, line: u16) {
        self.write(OP_F64, line);
        f64::to_be_bytes(float).iter().for_each(|b| self.write(*b, line));
    }

    pub fn write_f64_0(&mut self, float: f64) {
        self.write_f64(float, 0)
    }
    
    pub fn write_constant(&mut self, value: Value, line: u16) -> Result<(), String> {
        let constant_index = self.constants.len();
        
        if constant_index > 255 { 
            return Err("More than 255 constants".to_string());
        }
        
        self.constants.push(value);
        self.write(OP_CONTANT, line);
        self.write(constant_index as u8, line);
        Ok(())
    }
    
    pub fn load_constant(&self, index: u8) -> Value {
        self.constants[index as usize].clone()
    }
    
    pub fn get_line(&self, index: usize) -> u16 {
        self.lines[index]
    }
}