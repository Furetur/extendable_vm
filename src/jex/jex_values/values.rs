use crate::jex::syntax_exceptions::{InvalidFunctionChunk, NotFoundChunkForFunction};
use crate::jex::types::JexMachine;
use crate::machine::exceptions::types::Exception;

use crate::jex::jex_values::to_output_string::ToOutputString;
use std::convert::TryFrom;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone)]
pub enum JexValue {
    Null(JexNull),
    Int(i32),
    Bool(bool),
    Object(Rc<JexObject>),
    Function(JexFunction),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct JexNull;

#[derive(PartialEq, Eq, Clone)]
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
    pub fn as_int(&self) -> Option<i32> {
        if let JexValue::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        if let JexValue::Bool(bool) = self {
            Some(*bool)
        } else {
            None
        }
    }
    pub fn as_function(&self) -> Option<&JexFunction> {
        if let JexValue::Function(func) = self {
            Some(func)
        } else {
            None
        }
    }
    pub fn as_object(&self) -> Option<&JexObject> {
        if let JexValue::Object(obj) = self {
            Some(&**obj)
        } else {
            None
        }
    }
    pub fn as_string(&self) -> Option<&String> {
        let JexObject::String(string) = self.as_object()?;
        Some(string)
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

impl Debug for JexValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JexValue::Int(int) => write!(f, "{}", int),
            JexValue::Bool(bool) => write!(f, "{}", bool),
            JexValue::Null(_) => write!(f, "null"),
            JexValue::Function(func) => write!(f, "{}", func.to_output_string()),
            JexValue::Object(obj) => write!(f, "{:?}", &**obj),
        }
    }
}

impl Debug for JexObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JexObject::String(s) => write!(f, "\"{}\"", s),
        }
    }
}
