use std::rc::Rc;

pub type JexObject = Rc<RawObject>;

#[derive(Debug)]
pub enum RawObject {
    STRING(String),
}

pub fn object_to_string(obj: &JexObject) -> String {
    match &**obj {
        RawObject::STRING(s) => s.clone(),
        _ => String::from("<jex_object>")
    }
}
