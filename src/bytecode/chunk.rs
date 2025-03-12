use crate::bytecode::codes::OP_CONSTANT;

pub struct Chunk {
    pub(crate) code: Vec<u8>,
    lines: Vec<u16>
}

impl From<Vec<u8>> for Chunk {
    fn from(bytecode: Vec<u8>) -> Self {
        let mut lines = vec![];
        for _ in 0..bytecode.len() {
            lines.push(0);
        }
        Self { code: bytecode, lines }
    }
}

impl Chunk {
    pub fn new() -> Self {
        Self { code: Vec::new(), lines: Vec::new() }
    }

    pub fn write(&mut self, op: u8, line: u16) {
        self.code.push(op);
        self.lines.push(line);
    }

    pub fn write0(&mut self, op: u8) {
        self.write(op, 0);
    }

    pub fn write_constant_f64(&mut self, float: f64, line: u16) {
        self.write(OP_CONSTANT, line);
        f64::to_be_bytes(float).iter().for_each(|b| self.write(*b, line));
    }

    pub fn write_constant_f64_0(&mut self, float: f64) {
        self.write_constant_f64(float, 0)
    }
    
    pub fn get_line(&self, index: usize) -> u16 {
        self.lines[index]
    }
}