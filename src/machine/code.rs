use crate::machine::byte_readable::ByteReadable;
use crate::machine::errors::MachineError;
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
    ) -> Result<&Constant, MachineError> {
        let chunk = self
            .get_chunk(chunk_id)
            .ok_or(MachineError("Chunk not found".to_string()))?;
        if constant_id >= chunk.constants.len() {
            let message = format!(
                "Tried to read constant(id={}) from chunk(id={}), but there were only {} constants",
                constant_id,
                chunk_id,
                chunk.constants.len()
            );
            Err(MachineError(message))
        } else {
            Ok(&chunk.constants[constant_id])
        }
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
}
