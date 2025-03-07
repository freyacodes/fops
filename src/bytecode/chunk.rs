use crate::bytecode::codes::OP_CONSTANT;

pub struct Chunk {
    pub(crate) code: Vec<u8>
}

impl From<Vec<u8>> for Chunk {
    fn from(bytecode: Vec<u8>) -> Self {
        Self { code: bytecode }
    }
}

impl Chunk {
    pub fn new() -> Self {
        Self { code: Vec::new() }
    }

    pub fn write_simple(&mut self, op: u8) {
        self.code.push(op);
    }

    pub fn write_constant_f32(&mut self, float: f32) {
        self.code.push(OP_CONSTANT);
        f32::to_be_bytes(float).iter().for_each(|b| self.code.push(*b));
    }
}