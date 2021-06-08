use crate::jex::jex_values::values::{JexFunction, JexNull, JexObject, JexValue};

pub trait GetType {
    fn get_type(&self) -> String;
}

impl GetType for JexValue {
    fn get_type(&self) -> String {
        match self {
            JexValue::Int(int) => int.get_type(),
            JexValue::Bool(bool) => bool.get_type(),
            JexValue::Object(obj) => (&**obj).get_type(),
            JexValue::Function(func) => func.get_type(),
            JexValue::Null(null) => null.get_type(),
        }
    }
}

impl GetType for i32 {
    fn get_type(&self) -> String {
        "Int".to_string()
    }
}

impl GetType for bool {
    fn get_type(&self) -> String {
        "Boolean".to_string()
    }
}

impl GetType for JexObject {
    fn get_type(&self) -> String {
        "String".to_string()
    }
}

impl GetType for JexFunction {
    fn get_type(&self) -> String {
        match self {
            JexFunction::Function { .. } => "fn".to_string(),
            JexFunction::Script => "<script>".to_string(),
        }
    }
}

impl GetType for JexNull {
    fn get_type(&self) -> String {
        "null".to_string()
    }
}
