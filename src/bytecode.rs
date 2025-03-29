pub mod disassembler;
pub mod chunk;

fops_macros::opcodes! {
    codes:
    0x00 = OP_CONSTANT len 5,
    0x01 = OP_NIL,
    0x02 = OP_TRUE,
    0x03 = OP_FALSE,
    
    0x04 = OP_ADD,
    0x05 = OP_SUBTRACT,
    0x06 = OP_DIVIDE,
    0x07 = OP_MULTIPLY,
    0x08 = OP_NEGATE,
    
    0x09 = OP_NOT,
    0x10 = OP_EQUALS,
    0x11 = OP_NOT_EQUALS,
    0x12 = OP_LESS_THAN,
    0x13 = OP_LESS_THAN_OR_EQUALS,
    0x14 = OP_GREATER_THAN,
    0x15 = OP_GREATER_THAN_OR_EQUALS,
    
    0x16 = OP_RETURN
}