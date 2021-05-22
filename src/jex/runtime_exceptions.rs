use crate::machine::exceptions::types::{Exception, ExceptionType};

#[derive(Debug)]
pub struct TypeException(pub String);

impl From<TypeException> for Exception {
    fn from(exception: TypeException) -> Self {
        Exception {
            exception_type: ExceptionType::Runtime,
            name: "TypeException".to_string(),
            message: exception.0,
        }
    }
}

#[derive(Debug)]
pub struct ExpectedInstructionArgument;

impl From<ExpectedInstructionArgument> for Exception {
    fn from(exception: ExpectedInstructionArgument) -> Self {
        Exception {
            exception_type: ExceptionType::Runtime,
            name: "ExpectedInstructionArgument".to_string(),
            message: "".to_string(),
        }
    }
}
