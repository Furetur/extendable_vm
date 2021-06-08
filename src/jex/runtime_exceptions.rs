use crate::jex::jex_values::get_type::GetType;
use crate::jex::jex_values::values::JexValue;
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
    fn from(_exception: ExpectedInstructionArgument) -> Self {
        Exception {
            exception_type: ExceptionType::Runtime,
            name: "ExpectedInstructionArgument".to_string(),
            message: "".to_string(),
        }
    }
}

pub struct UnaryOperatorNotDefined;

impl From<UnaryOperatorNotDefined> for Exception {
    fn from(_: UnaryOperatorNotDefined) -> Self {
        Exception {
            exception_type: ExceptionType::Runtime,
            name: "UnaryOperatorNotDefined".to_string(),
            message: "Some unary operator not defined".to_string(),
        }
    }
}

pub struct OperatorNotDefined;

impl From<OperatorNotDefined> for Exception {
    fn from(_: OperatorNotDefined) -> Self {
        Exception {
            exception_type: ExceptionType::Runtime,
            name: "OperatorUndefined".to_string(),
            message: "Some binary operator not defined".to_string(),
        }
    }
}
