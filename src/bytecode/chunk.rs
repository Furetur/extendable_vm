use crate::bytecode::instructions::Instruction;
use crate::values::jex_values::JexValue;

#[derive(Debug, PartialEq)]
pub struct Chunk {
    pub constants: Vec<ChunkConstant>,
    pub code: Vec<Instruction>,
}

pub enum ChunkConstantOrdinal {
    Int = 0,
    String = 1,
}

#[derive(Debug, PartialEq)]
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
            ChunkConstant::STRING(str) => JexValue::from_string(str.clone()),
        }
    }
}
