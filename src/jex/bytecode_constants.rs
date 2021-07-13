use crate::jex::jex_values::values::{JexFunction, JexValue};
use crate::jex::runtime_exceptions::TypeException;
use crate::jex::types::JexMachine;
use crate::machine::code::{Chunk, Code};
use crate::machine::exceptions::types::Exception;
use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JexConstant {
    Int(i32),
    String(String),
    Function { chunk_id: usize },
}

pub enum JexConstantType {
    Int = 0,
    String = 1,
    Function = 2,
}

impl JexConstant {
    pub fn to_value(&self, machine: &JexMachine) -> Result<JexValue, Exception> {
        let value = match self {
            JexConstant::Int(i) => JexValue::Int(*i),
            JexConstant::String(str) => JexValue::from_string(str.clone()),
            JexConstant::Function { chunk_id } => {
                let func = JexFunction::from_code(machine, *chunk_id)?;
                JexValue::Function(func)
            }
        };
        Ok(value)
    }
    pub fn from_str(str: &str) -> JexConstant {
        JexConstant::String(str.to_string())
    }
    pub fn as_string(&self) -> Result<String, TypeException> {
        if let JexConstant::String(string) = self {
            Ok(string.clone())
        } else {
            Err(TypeException(format!(
                "Expected chunk constant {:?} to be string",
                self
            )))
        }
    }
    pub fn as_int(&self) -> Result<i32, TypeException> {
        if let JexConstant::Int(int) = self {
            Ok(*int)
        } else {
            Err(TypeException(format!(
                "Expected chunk constant {:?} to be int",
                self
            )))
        }
    }
}

impl Debug for Chunk<JexConstant> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Chunk")
            .field("constants", &self.constants)
            .field("code", &self.code)
            .finish()
    }
}

impl Debug for Code<JexConstant> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.chunks).finish()
    }
}
