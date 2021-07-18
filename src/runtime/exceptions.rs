use crate::exception::{Exception, ExceptionType};

#[derive(Debug)]
pub struct UnexpectedEndOfCode {
    pub chunk_id: usize,
}

impl From<UnexpectedEndOfCode> for Exception {
    fn from(exception: UnexpectedEndOfCode) -> Self {
        Exception {
            exception_type: ExceptionType::Runtime,
            name: "UnexpectedEndOfCode".to_string(),
            message: format!("Chunk #{}", exception.chunk_id),
        }
    }
}

#[derive(Debug)]
pub struct EmptyCallStack;

impl From<EmptyCallStack> for Exception {
    fn from(_: EmptyCallStack) -> Self {
        Exception {
            exception_type: ExceptionType::Runtime,
            name: "EmptyCallStack".to_string(),
            message: "".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct EmptyOperandStack;

impl From<EmptyOperandStack> for Exception {
    fn from(_: EmptyOperandStack) -> Self {
        Exception {
            exception_type: ExceptionType::Runtime,
            name: "EmptyOperandStack".to_string(),
            message: "Operand stack was empty".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct SlotOutOfBounds;

impl From<SlotOutOfBounds> for Exception {
    fn from(_: SlotOutOfBounds) -> Self {
        Exception {
            exception_type: ExceptionType::Runtime,
            name: "SlotOutOfBounds".to_string(),
            message: "Stack slot is out of bounds".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct UnknownOpCode(pub u8);

impl From<UnknownOpCode> for Exception {
    fn from(exception: UnknownOpCode) -> Self {
        Exception {
            exception_type: ExceptionType::Runtime,
            name: "UnknownOpCode".to_string(),
            message: format!("No instruction with opcode {} found", exception.0),
        }
    }
}

#[derive(Debug)]
pub struct ChunkNotFound(pub usize);

impl From<ChunkNotFound> for Exception {
    fn from(exception: ChunkNotFound) -> Self {
        Exception {
            exception_type: ExceptionType::Runtime,
            name: "ChunkNotFound".to_string(),
            message: format!("#{}", exception.0),
        }
    }
}

#[derive(Debug)]
pub struct ConstantNotFound(pub usize, pub usize);

impl From<ConstantNotFound> for Exception {
    fn from(exception: ConstantNotFound) -> Self {
        Exception {
            exception_type: ExceptionType::Runtime,
            name: "ConstantNotFound".to_string(),
            message: format!("#{}@{}", exception.0, exception.1),
        }
    }
}
