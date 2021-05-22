use crate::machine::exceptions::types::{Exception, ExceptionType};

#[derive(Debug)]
pub struct NotFoundChunkForFunction(pub usize);

impl From<NotFoundChunkForFunction> for Exception {
    fn from(exception: NotFoundChunkForFunction) -> Self {
        Exception {
            exception_type: ExceptionType::Static,
            name: "NotFoundChunkForFunction".to_string(),
            message: format!("Chunk #{}", exception.0),
        }
    }
}

#[derive(Debug)]
pub struct InvalidFunctionChunk(pub usize);

impl From<InvalidFunctionChunk> for Exception {
    fn from(exception: InvalidFunctionChunk) -> Self {
        Exception {
            exception_type: ExceptionType::Static,
            name: "InvalidFunctionChunk".to_string(),
            message: format!(
                "Chunk #{} cannot be interpreted as a function because it has no name or arity",
                exception.0
            ),
        }
    }
}
