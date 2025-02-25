pub const OP_CONSTANT: u8 = 0x00;
pub const OP_NEGATE: u8 = 0x01;
pub const OP_RETURN: u8 = 0x02;

pub const INSTRUCTION_NAMES: [&str; 3] = [
    "OP_LOAD",
    "OP_NEGATE",
    "OP_CONSTANT"
];

pub const INSTRUCTION_LENGTH: [u8; 3] = [
    5,
    1,
    1
];

pub const INSTRUCTION_COUNT: u8 = INSTRUCTION_NAMES.len() as u8;
