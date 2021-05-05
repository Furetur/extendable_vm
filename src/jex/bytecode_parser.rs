use crate::jex::bytecode_constants::{JexConstant, JexConstantType};
use crate::jex::bytecode_reader::{BytecodeReader, BytecodeReadingError};
use crate::jex::errors::BytecodeReadingError;
use crate::machine::code::{Chunk, Code};
use crate::machine::errors::CodeReadingError;

struct BytecodeParser;

struct ChunkParser {
    chunk_id: usize,
    constants: Vec<JexContant>,
}

impl BytecodeParser {
    fn new() -> BytecodeParser {
        BytecodeParser
    }
    fn parse(
        &mut self,
        reader: &mut BytecodeReader,
    ) -> Result<Code<JexConstant>, BytecodeReadingError> {
        let mut chunks: Vec<Chunk<JexConstant>> = vec![];
        let mut chunk_id = 0;
        while !reader.is_finished() {
            let mut chunk_parser = ChunkParser::new(chunk_id);
            chunk_id += 1;
            let chunk = chunk_parser.parse(reader)?;
            chunks.push(chunk);
        }
        Ok(Code { chunks })
    }
}

impl ChunkParser {
    pub fn new(chunk_id: usize) -> ChunkParser {
        ChunkParser {
            chunk_id,
            constants: Vec::new(),
        }
    }
    pub fn parse(
        &mut self,
        reader: &mut BytecodeReader,
    ) -> Result<Chunk<JexConstant>, BytecodeReadingError> {
        self.parse_constants(reader);
        let n_instruction_bytes = reader.read_u32("n_instruction_bytes in chunk")?;
        let code = reader.read_bytes(usize::from(n_instruction_bytes), "chunk code")?;

        Ok(Chunk {
            constants: self.constants.clone(),
            code,
        })
    }
    fn parse_constants(&mut self, reader: &mut BytecodeReader) -> Result<(), BytecodeReadingError> {
        let n_constants = reader.read_byte("chunk n_constants")?;
        for _ in 0..n_constants {
            let constant = self.read_constant(reader)?;
            self.constants.push(constant);
        }
        Ok(())
    }
    fn read_constant(
        &mut self,
        reader: &mut BytecodeReader,
    ) -> Result<JexConstant, BytecodeReadingError> {
        let constant_type = reader.read_byte("constant type")?;
        match constant_type {
            0 => self.read_int_constant(reader),
            1 => self.read_string_constant(reader),
            2 => self.read_function_constant(reader),
            i => {
                let message = format!(
                    "Unsupported constant type {} at position {}",
                    i,
                    reader.position()
                );
                Err(BytecodeReadingError(message))
            }
        }
    }
    fn read_int_constant(
        &mut self,
        reader: &mut BytecodeReader,
    ) -> Result<JexConstant, BytecodeReadingError> {
        let integer = reader.read_i32("int constant content")?;
        Ok(JexConstant::Int(integer))
    }
    fn read_string_constant(
        &mut self,
        reader: &mut BytecodeReader,
    ) -> Result<JexConstant, BytecodeReadingError> {
        let str_size = reader.read_u32("string constant length")?;
        let bytes = reader.read_bytes(usize::from(str_size), "string constant content")?;
        let string = String::from_utf8(bytes);
        match string {
            Ok(string) => Ok(JexConstant::String(string)),
            Err(..) => Err(BytecodeReadingError(
                "Could not decode utf8 string".to_string(),
            )),
        }
    }
    fn read_function_constant(
        &mut self,
        reader: &mut BytecodeReader,
    ) -> Result<JexConstant, BytecodeReadingError> {
        let chunk_id = usize::from(reader.read_byte("function constant chunk_id")?);
        Ok(JexConstant::Function { chunk_id })
    }
}
