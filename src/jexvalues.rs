use crate::jexobject::{JexObject, object_to_string, RawObject};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum JexValue {
    NULL,
    INT(i8),
    BOOLEAN(bool),
    // STRING(String),
    OBJECT(JexObject),
}


pub fn are_values_equal(x: &JexValue, y: &JexValue) -> bool {
    match (x, y) {
        (JexValue::NULL, JexValue::NULL) => true,
        (JexValue::INT(x), JexValue::INT(y)) => x == y,
        (JexValue::BOOLEAN(x), JexValue::BOOLEAN(y)) => x == y,
        _ => false
    }
}

impl JexValue {
    pub fn to_output_string(&self) -> String {
        match self {
            JexValue::NULL => String::from("null"),
            JexValue::INT(i) => format!("{}", i),
            JexValue::BOOLEAN(bool) => format!("{}", bool),
            // JexValue::STRING(str) => str.clone(),
            JexValue::OBJECT(obj) => object_to_string(obj),
        }
    }

    pub fn from_string(str: String) -> JexValue {
        let jex_obj = Rc::new(RawObject::STRING(str));
        JexValue::OBJECT(jex_obj)
    }
}
