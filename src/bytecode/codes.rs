pub const OP_CONSTANT: u8 = 0x00;
pub const OP_RETURN: u8 = 0x01;

pub const INSTRUCTION_NAMES: [&str; 2] = [
    "OP_LOAD",
    "OP_CONSTANT"
];

pub const INSTRUCTION_LENGTH: [u8; 2] = [
    5,
    1
];

pub const INSTRUCTION_COUNT: u8 = INSTRUCTION_NAMES.len() as u8;
