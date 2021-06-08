use crate::jex::runtime_exceptions::TypeException;
use crate::jex::syntax_exceptions::{InvalidFunctionChunk, NotFoundChunkForFunction};
use crate::jex::types::JexMachine;
use crate::machine::exceptions::types::Exception;

use std::convert::TryFrom;
use std::rc::Rc;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum JexValue {
    Null(JexNull),
    Int(i32),
    Bool(bool),
    Object(Rc<JexObject>),
    Function(JexFunction),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct JexNull;

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
    pub fn null() -> JexValue {
        JexValue::Null(JexNull)
    }
    pub fn from_string(string: String) -> JexValue {
        JexValue::Object(Rc::new(JexObject::String(string)))
    }
    pub fn as_int(&self) -> Result<i32, TypeException> {
        if let JexValue::Int(i) = self {
            Ok(i.clone())
        } else {
            Err(TypeException("Int".to_string()))
        }
    }
    pub fn as_bool(&self) -> Result<bool, TypeException> {
        if let JexValue::Bool(bool) = self {
            Ok(bool.clone())
        } else {
            Err(TypeException("Boolean".to_string()))
        }
    }
    pub fn as_function(&self) -> Result<&JexFunction, TypeException> {
        if let JexValue::Function(func) = self {
            Ok(func)
        } else {
            Err(TypeException("Function".to_string()))
        }
    }
    pub fn as_object(&self) -> Result<&JexObject, TypeException> {
        if let JexValue::Object(obj) = self {
            Ok(&**obj)
        } else {
            Err(TypeException("Object".to_string()))
        }
    }
    pub fn as_string(&self) -> Result<&String, TypeException> {
        let JexObject::String(string) = self.as_object()?;
        Ok(string)
    }
}

impl JexFunction {
    pub fn from_code(machine: &JexMachine, chunk_id: usize) -> Result<JexFunction, Exception> {
        let chunk = machine
            .code
            .get_chunk(chunk_id)
            .ok_or(NotFoundChunkForFunction(chunk_id))?;
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
            Err(Exception::from(InvalidFunctionChunk(chunk_id)))
        }
    }
}
