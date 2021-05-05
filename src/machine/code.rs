use crate::machine::errors::CodeReadingError;
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
    ) -> Result<&Constant, CodeReadingError> {
        let chunk = self.get_chunk(chunk_id)?;
        if constant_id >= chunk.constants.len() {
            let message = format!(
                "Tried to read constant(id={}) from chunk(id={}), but there were only {} constants",
                constant_id,
                chunk_id,
                chunk.constants.len()
            );
            Err(CodeReadingError(message))
        } else {
            Ok(&chunk.constants[constant_id])
        }
    }

    pub fn read(
        &self,
        instruction_pointer: &mut InstructionPointer,
    ) -> Result<Option<u8>, CodeReadingError> {
        let chunk = self.get_chunk(instruction_pointer.chunk_id)?;
        Ok(instruction_pointer.read_and_advance(chunk))
    }

    pub fn read_for(
        &self,
        instruction_pointer: &mut InstructionPointer,
        for_what: &str,
    ) -> Result<u8, CodeReadingError> {
        if let Some(byte) = self.read(instruction_pointer) {
            Ok(byte)
        } else {
            Err(CodeReadingError(format!(
                "Unexpected end of bytes while reading {}",
                for_what
            )))
        }
    }

    pub fn read_n_for(
        &self,
        n: u8,
        instruction_pointer: &mut InstructionPointer,
        for_what: &str,
    ) -> Result<Vec<u8>, CodeReadingError> {
        let mut result: Vec<u8> = Vec::new();
        for _ in 0..n {
            let byte = self.read_for(instruction_pointer, &for_what)?;
            result.push(byte);
        }
        Ok(result)
    }

    fn get_chunk(&self, chunk_id: usize) -> Result<&Chunk<Constant>, CodeReadingError> {
        if chunk_id >= self.chunks.len() {
            let message = format!(
                "Cannot get chunk with id={} because there are only {} chunks",
                chunk_id,
                self.chunk.len()
            );
            Err(CodeReadingError(message))
        } else {
            Ok(&self.chunks[chunk_id])
        }
    }
}
