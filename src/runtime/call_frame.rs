use crate::runtime::instruction_pointer::InstructionPointer;
use std::fmt;
use std::fmt::{Display, Formatter};

pub struct CallFrame {
    pub chunk_id: usize,
    pub name: String,
    pub instruction_pointer: InstructionPointer,
    pub start_slot: usize,
}

impl CallFrame {
    pub(crate) fn new(chunk_id: usize, name: String, start_slot: usize) -> CallFrame {
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
