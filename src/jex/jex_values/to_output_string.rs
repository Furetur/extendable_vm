use crate::jex::jex_values::values::{JexFunction, JexNull, JexObject, JexValue};
use std::rc::Rc;

pub trait ToOutputString {
    fn to_output_string(&self) -> String;
}

impl ToOutputString for JexValue {
    fn to_output_string(&self) -> String {
        match self {
            JexValue::Null(null) => null.to_output_string(),
            JexValue::Int(int) => int.to_output_string(),
            JexValue::Bool(bool) => bool.to_output_string(),
            JexValue::Function(func) => func.to_output_string(),
            JexValue::Object(obj) => obj.to_output_string(),
        }
    }
}

impl ToOutputString for JexNull {
    fn to_output_string(&self) -> String {
        "null".to_string()
    }
}

impl ToOutputString for i32 {
    fn to_output_string(&self) -> String {
        self.to_string()
    }
}

impl ToOutputString for bool {
    fn to_output_string(&self) -> String {
        self.to_string()
    }
}

impl ToOutputString for Rc<JexObject> {
    fn to_output_string(&self) -> String {
        (&**self).to_output_string()
    }
}

impl ToOutputString for JexObject {
    fn to_output_string(&self) -> String {
        let JexObject::String(str) = self;
        str.clone()
    }
}

impl ToOutputString for JexFunction {
    fn to_output_string(&self) -> String {
        match self {
            JexFunction::Script => "<script>".to_string(),
            JexFunction::Function { name, arity, .. } => {
                format!("function {}({} params)", name, arity)
            }
        }
    }
}
