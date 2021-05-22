use crate::machine::exceptions::types::{Exception, ExceptionType};

#[derive(Debug)]
pub struct EmptyCode;

impl From<EmptyCode> for Exception {
    fn from(_: EmptyCode) -> Self {
        Exception {
            exception_type: ExceptionType::Static,
            name: "EmptyCode".to_string(),
            message: "Found 0 code chunks".to_string(),
        }
    }
}
