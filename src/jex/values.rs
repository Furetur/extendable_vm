use std::rc::Rc;
use crate::machine::errors::TypeError;

#[derive(Eq, Debug, Clone)]
pub enum JexValue {
    Null,
    Int(i32),
    Bool(bool),
    Object(Rc<JexObject>),
    Function(JexFunction)
}

#[derive(Eq, Debug, Clone)]
pub enum JexObject {
    String(String),
}

#[derive(Eq, Debug, Clone)]
pub enum JexFunction {
    Script,
    Function { arity: usize, chunk_id: usize, name: String }
}

impl JexValue {
    pub fn to_output_string(&self) -> String {
        match self {
            JexValue::Null => "null".to_string(),
            JexValue::Int(int) => int.to_string(),
            JexValue::Bool(bool) => bool.to_string(),
            JexValue::Object(obj) => **obj.to_output_string(),
            JexValue::Function(func) => func.to_output_string()
        }
    }
    pub fn as_int(&self) -> Result<i32, TypeError> {
        if let JexValue::Int(i) = self {
            Ok(i.clone())
        } else {
            Err(TypeError(format!("Expected {} to be int", self.to_output_string())))
        }
    }
    pub fn as_bool(&self) -> Result<bool, TypeError> {
        if let JexValue::Bool(bool) = self {
            Ok(bool.clone())
        } else {
            Err(TypeError(format!("Expected {} to be bool", self.to_output_string())))
        }
    }
    pub fn as_function(&self) -> Result<&JexFunction, TypeError> {
        if let JexValue::Function(func) = self {
            Ok(func)
        } else {
            Err(TypeError(format!("Expected {} to be a function", self.to_output_string())))
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
    pub fn to_output_string(&self) -> String {
        if let JexFunction::Function { name, arity, ..} = self {
            format!("function {}({} params)", name, arity)
        } else {
            "<script>".to_string()
        }
    }
}
