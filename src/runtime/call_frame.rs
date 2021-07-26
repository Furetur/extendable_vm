use crate::runtime::instruction_pointer::InstructionPointer;
use std::fmt;
use std::fmt::{Display, Formatter};

/// A struct that stores information about an active function call.
///
/// CallFrame stores information about a function call that has not returned yet.
/// `chunk_id` is the id of the chunk that defines the function.
/// `name` -- the name of the function.
/// `instruction_pointer` -- a pointer to a certain point in code which the function is executing.
/// `start_slot` -- the index in the operand stack at which the call frame starts.
pub struct CallFrame {
    pub chunk_id: usize, // TODO: remove this. chunk_id is already stored in the pointer
    pub name: String,
    pub instruction_pointer: InstructionPointer,
    pub start_slot: usize,
}

impl CallFrame {
    pub fn new(chunk_id: usize, name: String, start_slot: usize) -> CallFrame {
        CallFrame {
            chunk_id,
            name,
            instruction_pointer: InstructionPointer::new(chunk_id),
            start_slot,
        }
    }
}

impl Display for CallFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (#{}:{})",
            self.name, self.chunk_id, self.instruction_pointer.instruction_pointer
        )
    }
}
