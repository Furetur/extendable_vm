use crate::byte_readable::ByteReadable;
use crate::exception::Exception;
use crate::runtime::exceptions::ConstantNotFound;
use crate::InstructionPointer;
use std::fmt;
use std::fmt::{Debug, Formatter};

pub struct Chunk<Constant> {
    pub constants: Vec<Constant>,
    pub code: Vec<u8>,
}

pub struct Code<Constant> {
    pub chunks: Vec<Chunk<Constant>>,
}

impl<Constant> Code<Constant> {
    pub fn get_constant(
        &self,
        chunk_id: usize,
        constant_id: usize,
    ) -> Result<&Constant, Exception> {
        self.get_chunk(chunk_id)
            .and_then(|chunk| chunk.constants.get(constant_id))
            .ok_or_else(|| Exception::from(ConstantNotFound(chunk_id, constant_id)))
    }

    pub fn get_chunk(&self, chunk_id: usize) -> Option<&Chunk<Constant>> {
        self.chunks.get(chunk_id)
    }
}

impl<Constant> ByteReadable<InstructionPointer> for Code<Constant> {
    fn read(&self, ptr: &mut InstructionPointer) -> Option<u8> {
        let chunk = self.get_chunk(ptr.chunk_id)?;
        ptr.read_and_advance(chunk)
    }

    fn has_next(&self, ptr: &InstructionPointer) -> bool {
        let chunk = self.get_chunk(ptr.chunk_id).unwrap();
        ptr.instruction_pointer < chunk.code.len()
    }
}

impl<Constant: Debug> Debug for Chunk<Constant> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Chunk")
            .field("constants", &self.constants)
            .field("code", &self.code)
            .finish()
    }
}

impl<Constant: Debug> Debug for Code<Constant> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.chunks).finish()
    }
}
