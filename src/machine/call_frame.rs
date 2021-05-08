use crate::machine::instruction_pointer::InstructionPointer;

pub struct CallFrame {
    pub chunk_id: usize,
    pub instruction_pointer: InstructionPointer,
    pub(crate) start_slot: usize,
}

impl CallFrame {
    pub(crate) fn new(chunk_id: usize, start_slot: usize) -> CallFrame {
        CallFrame {
            chunk_id,
            instruction_pointer: InstructionPointer::new(chunk_id),
            start_slot,
        }
    }
}
