use crate::jexvalues::{JexValue};

#[derive(Debug)]
pub enum Instruction {
    // builtin
    PRINT,
    // constant instructions and literal instructions
    CONSTANT(usize),
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
    pub constants: Vec<ChunkConstant>,
    pub code: Vec<Instruction>,
}

#[derive(Debug)]
pub enum ChunkConstant {
    INT(i8),
    STRING(String),
}

impl ChunkConstant {
    pub fn from_str(str: &str) -> ChunkConstant {
        ChunkConstant::STRING(String::from(str))
    }

    pub fn to_jex_value(&self) -> JexValue {
        match self {
            ChunkConstant::INT(n) => JexValue::INT(*n),
            ChunkConstant::STRING(str) => JexValue::from_string(str.clone())
        }
    }
}

