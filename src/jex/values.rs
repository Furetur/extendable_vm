use crate::jex::types::JexMachine;
use crate::machine::errors::MachineError;
use std::rc::Rc;
use std::convert::TryFrom;
use crate::machine::machine::Machine;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum JexValue {
    Null,
    Int(i32),
    Bool(bool),
    Object(Rc<JexObject>),
    Function(JexFunction),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum JexObject {
    String(String),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum JexFunction {
    Script,
    Function {
        arity: usize,
        chunk_id: usize,
        name: String,
    },
}

impl JexValue {
    pub fn to_output_string(&self) -> String {
        match self {
            JexValue::Null => "null".to_string(),
            JexValue::Int(int) => int.to_string(),
            JexValue::Bool(bool) => bool.to_string(),
            JexValue::Object(obj) => (**obj).to_output_string(),
            JexValue::Function(func) => func.to_output_string(),
        }
    }
    pub fn from_string(string: String) -> JexValue {
        JexValue::Object(Rc::new(JexObject::String(string)))
    }
    pub fn as_int(&self) -> Result<i32, MachineError> {
        if let JexValue::Int(i) = self {
            Ok(i.clone())
        } else {
            Err(MachineError(format!(
                "Expected {} to be int",
                self.to_output_string()
            )))
        }
    }
    pub fn as_bool(&self) -> Result<bool, MachineError> {
        if let JexValue::Bool(bool) = self {
            Ok(bool.clone())
        } else {
            Err(MachineError(format!(
                "Expected {} to be bool",
                self.to_output_string()
            )))
        }
    }
    pub fn as_function(&self) -> Result<&JexFunction, MachineError> {
        if let JexValue::Function(func) = self {
            Ok(func)
        } else {
            Err(MachineError(format!(
                "Expected {} to be a function",
                self.to_output_string()
            )))
        }
    }
    pub fn as_object(&self) -> Result<&JexObject, MachineError> {
        if let JexValue::Object(obj) = self {
            Ok(&**obj)
        } else {
            Err(MachineError(format!(
                "Expected {} to be an object",
                self.to_output_string()
            )))
        }
    }
    pub fn as_string(&self) -> Result<&String, MachineError> {
        if let JexObject::String(string) = self.as_object()? {
            Ok(string)
        } else {
            Err(MachineError(format!(
                "Expected {} to be a string",
                self.to_output_string()
            )))
        }
    }
}

impl JexObject {
    pub fn to_output_string(&self) -> String {
        let JexObject::String(str) = self;
        str.clone()
    }
}

impl JexFunction {
    pub fn from_code(
        machine: &JexMachine,
        chunk_id: usize,
    ) -> Result<JexFunction, MachineError> {
        let chunk = machine.code.get_chunk(chunk_id)?;
        let name = chunk.constants[0].as_string()?;
        let read_arity = chunk.constants[1].as_int()?;
        let arity = usize::try_from(read_arity);
        if let Ok(usize) = arity {
            Ok(JexFunction::Function {
                chunk_id,
                name,
                arity: usize,
            })
        } else {
            Err(MachineError(format!(
                "Could not read function arity {} in the chunk #{}",
                read_arity, chunk_id
            )))
        }
    }
    pub fn to_output_string(&self) -> String {
        if let JexFunction::Function { name, arity, .. } = self {
            format!("function {}({} params)", name, arity)
        } else {
            "<script>".to_string()
        }
    }
}
