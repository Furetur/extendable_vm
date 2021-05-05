use crate::jex::bytecode_reader::BytecodeReader;
use crate::machine::errors::TypeError;
use crate::jex::values::JexValue;
use crate::jex::types::JexMachine;

#[derive(Debug)]
pub enum JexConstant {
    Int(i32),
    String(String),
    Function { chunk_id: usize }
}

pub enum JexConstantType {
    Int = 0,
    String = 1,
    Function = 2
}

impl JexConstant {
    pub fn to_value(&self, machine: &JexMachine) -> JexValue {

    }
    pub fn as_string(&self) -> Result<String, TypeError> {
        if let JexConstant::String(string) = self {
            Ok(string.clone())
        } else {
            Err(TypeError(format!("Expected constant {:?} to be string", self)));
        }
    }
}
