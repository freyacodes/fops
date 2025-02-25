pub const OP_CONSTANT: u8 = 0x00;
pub const OP_NEGATE: u8 = 0x01;
pub const OP_ADD: u8 = 0x02;
pub const OP_SUBTRACT: u8 = 0x03;
pub const OP_DIVIDE: u8 = 0x04;
pub const OP_MULTIPLY: u8 = 0x05;
pub const OP_RETURN: u8 = 0x06;

pub const INSTRUCTION_TYPES: usize = 7;

pub const INSTRUCTION_NAMES: [&str; INSTRUCTION_TYPES] = [
    "OP_LOAD",
    "OP_ADD",
    "OP_SUBTRACT",
    "OP_DIVIDE",
    "OP_MULTIPLY",
    "OP_NEGATE",
    "OP_CONSTANT"
];

pub const INSTRUCTION_LENGTH: [u8; INSTRUCTION_TYPES] = [
    5,
    1,
    1,
    1,
    1,
    1,
    1
];

pub const INSTRUCTION_COUNT: u8 = INSTRUCTION_NAMES.len() as u8;
