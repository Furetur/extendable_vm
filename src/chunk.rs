#[derive(Debug)]
pub enum Instruction {
    // constant instructions and literal instructions
    CONSTANT(u8),
    NULL,
    TRUE,
    FALSE,
    // logic operators
    NOT,
    // comparison operators
    EQUAL,
    GREATER,
    LESS,
    // arithmetic operators
    NEGATE,
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
}
#[derive(Debug)]
pub struct Chunk {
    pub constants: Vec<u8>,
    pub code: Vec<Instruction>,
}
