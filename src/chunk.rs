use crate::jexvalues::{JexValue};

#[derive(Debug)]
pub enum Instruction {
    // builtin
    Print,
    // Variables
    DefineGlobal(usize),
    GetGlobal(usize),
    SetLocal(usize),
    GetLocal(usize),
    Pop(),
    // constant instructions and literal instructions
    Constant(usize),
    Null,
    True,
    False,
    // logic operators
    Not,
    // comparison operators
    Equal,
    Greater,
    Less,
    // arithmetic operators
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
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

