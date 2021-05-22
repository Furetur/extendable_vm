use crate::machine::byte_readable::ByteReadable;
use crate::machine::exceptions::runtime_exceptions::{ChunkNotFound, ConstantNotFound};
use crate::machine::exceptions::types::Exception;
use crate::machine::instruction_pointer::InstructionPointer;

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
            .ok_or(Exception::from(ConstantNotFound(chunk_id, constant_id)))
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
