pub const OP_CONSTANT: u8 = 0x00;
pub const OP_NIL: u8 = 0x01;
pub const OP_TRUE: u8 = 0x02;
pub const OP_FALSE: u8 = 0x03;
pub const OP_ADD: u8 = 0x04;
pub const OP_SUBTRACT: u8 = 0x05;
pub const OP_DIVIDE: u8 = 0x06;
pub const OP_MULTIPLY: u8 = 0x07;
pub const OP_NOT: u8 = 0x08;
pub const OP_NEGATE: u8 = 0x09;
pub const OP_RETURN: u8 = 0x10;

pub const INSTRUCTION_TYPES: usize = 11;

pub const INSTRUCTION_NAMES: [&str; INSTRUCTION_TYPES] = [
    "OP_CONSTANT",
    "OP_NIL",
    "OP_TRUE",
    "OP_FALSE",
    "OP_ADD",
    "OP_SUBTRACT",
    "OP_DIVIDE",
    "OP_MULTIPLY",
    "OP_NOT",
    "OP_NEGATE",
    "OP_RETURN",
];

pub const INSTRUCTION_LENGTH: [u8; INSTRUCTION_TYPES] = [
    5,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
];

pub const INSTRUCTION_COUNT: u8 = INSTRUCTION_NAMES.len() as u8;
