use std::collections::HashMap;
use crate::jexobject::{JexObject, RawObject};
use std::rc::Rc;
use crate::jexvalues::JexValue;

pub struct StringInterner {
    table: HashMap<String, JexObject>
}

impl StringInterner {
    pub fn new() -> StringInterner {
        StringInterner {
            table: HashMap::new()
        }
    }

    pub fn get_string_object(&mut self, str: String) -> JexObject {
        if self.table.contains_key(&str) {
            self.table.get(&str).unwrap().clone()
        } else {
            let raw_obj = RawObject::STRING(str.clone());
            let obj = Rc::new(raw_obj);
            self.table.insert(str, obj.clone());
            obj
        }
    }
    pub fn get_string_value(&mut self, str: String) -> JexValue {
        JexValue::OBJECT(self.get_string_object(str))
    }
}
