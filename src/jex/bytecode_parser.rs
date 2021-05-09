use crate::jex::bytecode_constants::JexConstant;
use crate::jex::bytecode_reader::BytecodeReader;
use crate::machine::code::{Chunk, Code};
use crate::machine::errors::MachineError;

pub struct BytecodeParser;

struct ChunkParser {
    chunk_id: usize,
    constants: Vec<JexConstant>,
}

impl BytecodeParser {
    pub fn new() -> BytecodeParser {
        BytecodeParser
    }
    pub fn parse(
        &mut self,
        reader: &mut BytecodeReader,
    ) -> Result<Code<JexConstant>, MachineError> {
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
    ) -> Result<Chunk<JexConstant>, MachineError> {
        self.parse_constants(reader);
        let n_instruction_bytes = reader.read_u16("n_instruction_bytes in chunk")?;
        let code = reader.read_bytes(usize::from(n_instruction_bytes), "chunk code")?;

        Ok(Chunk {
            constants: self.constants.clone(),
            code,
        })
    }
    fn parse_constants(&mut self, reader: &mut BytecodeReader) -> Result<(), MachineError> {
        let n_constants = reader.read_byte("chunk n_constants")?;
        for _ in 0..n_constants {
            let constant = self.read_constant(reader)?;
            self.constants.push(constant);
        }
        Ok(())
    }
    fn read_constant(&mut self, reader: &mut BytecodeReader) -> Result<JexConstant, MachineError> {
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
                Err(MachineError(message))
            }
        }
    }
    fn read_int_constant(
        &mut self,
        reader: &mut BytecodeReader,
    ) -> Result<JexConstant, MachineError> {
        let integer = reader.read_i32("int constant content")?;
        Ok(JexConstant::Int(integer))
    }
    fn read_string_constant(
        &mut self,
        reader: &mut BytecodeReader,
    ) -> Result<JexConstant, MachineError> {
        let str_size = reader.read_u16("string constant length")?;
        let bytes = reader.read_bytes(usize::from(str_size), "string constant content")?;
        let string = String::from_utf8(bytes);
        match string {
            Ok(string) => Ok(JexConstant::String(string)),
            Err(..) => Err(MachineError("Could not decode utf8 string".to_string())),
        }
    }
    fn read_function_constant(
        &mut self,
        reader: &mut BytecodeReader,
    ) -> Result<JexConstant, MachineError> {
        let chunk_id = usize::from(reader.read_byte("function constant chunk_id")?);
        Ok(JexConstant::Function { chunk_id })
    }
}

#[cfg(test)]
mod tests {
    use crate::jex::bytecode_constants::JexConstant;
    use crate::jex::bytecode_parser::BytecodeParser;
    use crate::jex::bytecode_reader::BytecodeReader;
    use crate::machine::code::{Chunk, Code};
    use std::convert::TryFrom;

    fn parse(bytes: Vec<u8>) -> Code<JexConstant> {
        let mut reader = BytecodeReader::new(bytes);
        BytecodeParser::new().parse(&mut reader).unwrap()
    }

    #[test]
    fn test_should_parse_1_chunk_without_constants_and_code() {
        let n_instruction: u16 = 0;
        let bytes: Vec<u8> = vec![
            0, // 0 constants
            n_instruction.to_le_bytes()[0],
            n_instruction.to_le_bytes()[1], // 0 instructions
        ];
        let code = parse(bytes);
        assert_eq!(1, code.chunks.len());
        let chunk = code.chunks.first().unwrap();
        assert_eq!(0, chunk.code.len());
        assert_eq!(0, chunk.constants.len())
    }

    #[test]
    fn test_should_parse_2_chunks_without_constants_and_code() {
        let n_instruction: u16 = 0;
        let bytes: Vec<u8> = vec![
            0, // 0 constants
            n_instruction.to_le_bytes()[0],
            n_instruction.to_le_bytes()[1], // 0 instructions
            0,                              // 0 constants
            n_instruction.to_le_bytes()[0],
            n_instruction.to_le_bytes()[1], // 0 instructions
        ];
        let code = parse(bytes);
        assert_eq!(2, code.chunks.len());
        assert!(code.chunks.iter().all(|chunk| chunk.code.len() == 0));
        assert!(code.chunks.iter().all(|chunk| chunk.constants.len() == 0));
    }

    #[test]
    fn test_should_parse_1_chunk_without_constants_and_with_code() {
        let n_instruction: u16 = 3;
        let bytes: Vec<u8> = vec![
            0,                              // 0 constants
            n_instruction.to_le_bytes()[0], // 3 instructions
            n_instruction.to_le_bytes()[1],
            1,
            2,
            3, // instructions
        ];
        let code = parse(bytes);
        assert_eq!(1, code.chunks.len());
        let chunk = code.chunks.first().unwrap();
        assert_eq!(0, chunk.constants.len());
        assert_eq!(vec![1, 2, 3], chunk.code.clone());
    }

    #[test]
    fn test_should_parse_1_chunk_with_1_int_constant_and_without_code() {
        let n_instruction: u16 = 0;
        let int: i32 = 157;
        let int_bytes = int.to_le_bytes();
        let bytes: Vec<u8> = vec![
            1, // 1 constant1
            0, // int
            int_bytes[0],
            int_bytes[1],
            int_bytes[2],
            int_bytes[3],
            n_instruction.to_le_bytes()[0],
            n_instruction.to_le_bytes()[1], // 0 instructions
        ];
        let code = parse(bytes);
        assert_eq!(1, code.chunks.len());
        let chunk = code.chunks.first().unwrap();
        assert_eq!(0, chunk.code.len());
        assert_eq!(
            JexConstant::Int(int),
            chunk.constants.first().unwrap().clone()
        )
    }
    #[test]
    fn test_should_parse_1_chunk_with_1_string_constant_and_without_code() {
        let n_instruction: u16 = 0;
        let string = "string string :)".to_string();
        let string_bytes = string.clone().into_bytes();
        let length = u16::try_from(string_bytes.len()).unwrap();
        let mut bytes: Vec<u8> = vec![
            1, // 1 constant1
            1, // int
            length.to_le_bytes()[0],
            length.to_le_bytes()[1],
        ];
        for string_byte in string_bytes {
            bytes.push(string_byte)
        }
        bytes.push(n_instruction.to_le_bytes()[0]);
        bytes.push(n_instruction.to_le_bytes()[1]);

        let code = parse(bytes);
        assert_eq!(1, code.chunks.len());
        let chunk = code.chunks.first().unwrap();
        assert_eq!(0, chunk.code.len());
        assert_eq!(
            JexConstant::String(string.clone()),
            chunk.constants.first().unwrap().clone()
        )
    }
    #[test]
    fn test_should_parse_1_chunk_with_1_fn_constant_and_without_code() {
        let n_instruction: u16 = 0;
        let int: i32 = 157;
        let int_bytes = int.to_le_bytes();
        let bytes: Vec<u8> = vec![
            1, // 1 constant1
            2, // fn
            0,
            n_instruction.to_le_bytes()[0],
            n_instruction.to_le_bytes()[1], // 0 instructions
        ];
        let code = parse(bytes);
        assert_eq!(1, code.chunks.len());
        let chunk = code.chunks.first().unwrap();
        assert_eq!(0, chunk.code.len());
        assert_eq!(
            JexConstant::Function { chunk_id: 0 },
            chunk.constants.first().unwrap().clone()
        )
    }
}
