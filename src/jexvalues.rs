use crate::jexobject::{JexObject, object_to_string, RawObject};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum JexValue {
    NULL,
    INT(i8),
    BOOLEAN(bool),
    OBJECT(JexObject),
}


pub fn are_values_equal(x: &JexValue, y: &JexValue) -> bool {
    match (x, y) {
        (JexValue::NULL, JexValue::NULL) => true,
        (JexValue::INT(x), JexValue::INT(y)) => x == y,
        (JexValue::BOOLEAN(x), JexValue::BOOLEAN(y)) => x == y,
        (JexValue::OBJECT(o1), JexValue::OBJECT(o2)) => o1 == o2,
        _ => false
    }
}

impl JexValue {
    pub fn to_output_string(&self) -> String {
        match self {
            JexValue::NULL => String::from("null"),
            JexValue::INT(i) => format!("{}", i),
            JexValue::BOOLEAN(bool) => format!("{}", bool),
            JexValue::OBJECT(obj) => object_to_string(obj),
        }
    }

    pub fn from_string(str: String) -> JexValue {
        let jex_obj = Rc::new(RawObject::STRING(str));
        JexValue::OBJECT(jex_obj)
    }

    pub fn as_int(&self) -> i8 {
        if let JexValue::INT(n) = self {
            *n
        } else {
            panic!("Expected {:?} to be an INT", self);
        }
    }

    pub fn as_bool(&self) -> bool {
        if let JexValue::BOOLEAN(b) = self {
            *b
        } else {
            panic!("Expected {:?} to be a BOOL", self);
        }
    }

    pub fn as_str(&self) -> &str {
        if let JexValue::OBJECT(o) = self {
            if let RawObject::STRING(s) = &**o {
                s.as_str()
            } else {
                panic!("Expected {:?} to be JexObject<String>", self);
            }
        } else {
            panic!("Expected {:?} to be JexObject<String>", self);
        }
    }
}
