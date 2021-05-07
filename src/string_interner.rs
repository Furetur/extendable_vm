use crate::jex::values::{JexObject, JexValue};
use std::collections::HashMap;
use std::rc::Rc;

pub struct StringInterner {
    table: HashMap<String, Rc<JexObject>>,
}

impl StringInterner {
    pub fn new() -> StringInterner {
        StringInterner {
            table: HashMap::new(),
        }
    }

    pub fn get_string_object(&mut self, str: String) -> Rc<JexObject> {
        if self.table.contains_key(&str) {
            self.table.get(&str).unwrap().clone()
        } else {
            let raw_obj = JexObject::String(str.clone());
            let obj = Rc::new(raw_obj);
            self.table.insert(str, obj.clone());
            obj
        }
    }
    pub fn get_string_value(&mut self, str: String) -> JexValue {
        JexValue::Object(self.get_string_object(str))
    }
}
