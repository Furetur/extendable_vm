use crate::exception::{Exception, ExceptionType};

pub struct EmptyCode;

impl From<EmptyCode> for Exception {
    fn from(_: EmptyCode) -> Self {
        Exception {
            exception_type: ExceptionType::Static,
            name: "EmptyCode".to_string(),
            message: "Code cannot be empty (have 0 bytes)".to_string(),
        }
    }
}

pub struct ChunkParsingError(pub usize, pub Exception);

impl From<ChunkParsingError> for Exception {
    fn from(error: ChunkParsingError) -> Self {
        Exception {
            exception_type: ExceptionType::Static,
            name: "ChunkParsingError".to_string(),
            message: format!("Could not parse chunk #{}: {}", error.0, error.1),
        }
    }
}

pub struct CodeEndedAt(pub String);

impl From<CodeEndedAt> for Exception {
    fn from(exception: CodeEndedAt) -> Self {
        Exception {
            exception_type: ExceptionType::Static,
            name: "CodeEndedAt".to_string(),
            message: format!("Code ended while reading {}", exception.0),
        }
    }
}

pub struct UnknownConstantType(pub u8);

impl From<UnknownConstantType> for Exception {
    fn from(exception: UnknownConstantType) -> Self {
        Exception {
            exception_type: ExceptionType::Static,
            name: "UnknownConstantType".to_string(),
            message: format!("Unknown constant with type {}", exception.0),
        }
    }
}

pub struct IllegalConstant(pub u8, pub Exception);

impl From<IllegalConstant> for Exception {
    fn from(exception: IllegalConstant) -> Self {
        Exception {
            exception_type: ExceptionType::Static,
            name: "IllegalConstant".to_string(),
            message: format!(
                "Could not parse constant with type {}: {}",
                exception.0, exception.1
            ),
        }
    }
}
