use crate::byte_readable::ByteReadable;
use crate::code::{Chunk, Code};
use crate::exception::Exception;
use crate::parsing::constant_parser::ConstantParserTable;
use crate::parsing::exceptions::{
    ChunkParsingError, CodeEndedAt, EmptyCode, IllegalConstant, UnknownConstantType,
};
use crate::parsing::raw_bytes::{RawBytes, RawBytesPointer};

/// Parses code from raw bytes using a table of constant parsers.
pub struct CodeParser<'a, Constant> {
    parsers: &'a ConstantParserTable<'a, Constant>,
}

impl<'a, Constant> CodeParser<'a, Constant> {
    pub fn new(parsers: &'a ConstantParserTable<'a, Constant>) -> CodeParser<'a, Constant> {
        CodeParser { parsers }
    }
    pub fn parse(&self, bytes: &RawBytes) -> Result<Code<Constant>, Exception> {
        let mut chunks: Vec<Chunk<Constant>> = vec![];
        let mut ptr = RawBytesPointer::new();
        let mut chunk_id: usize = 0;
        while bytes.has_next(&ptr) {
            let chunk = self
                .parse_chunk(bytes, &mut ptr)
                .map_err(|err| ChunkParsingError(chunk_id, err))?;
            chunks.push(chunk);
            chunk_id += 1;
        }
        if chunks.is_empty() {
            Err(Exception::from(EmptyCode))
        } else {
            Ok(Code { chunks })
        }
    }
    fn parse_chunk(
        &self,
        bytes: &RawBytes,
        ptr: &mut RawBytesPointer,
    ) -> Result<Chunk<Constant>, Exception> {
        let mut result_constants: Vec<Constant> = vec![];
        let n_constants = bytes
            .read(ptr)
            .ok_or_else(|| CodeEndedAt("n_constants".to_string()))?;
        for _ in 0..n_constants {
            let constant_type = bytes
                .read(ptr)
                .ok_or_else(|| CodeEndedAt("constant_type".to_string()))?;
            let constant_parser = self
                .parsers
                .get_parser(constant_type)
                .ok_or(UnknownConstantType(constant_type))?;
            let constant = (constant_parser.parser_fn)(bytes, ptr)
                .map_err(|e| IllegalConstant(constant_parser.constant_type, e))?;
            result_constants.push(constant);
        }
        let n_code_bytes = bytes
            .read_u16(ptr)
            .ok_or_else(|| CodeEndedAt("n_code_bytes".to_string()))?;
        let code = bytes
            .read_n(ptr, usize::from(n_code_bytes))
            .ok_or_else(|| CodeEndedAt("code".to_string()))?;
        Ok(Chunk {
            constants: result_constants,
            code,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::byte_readable::ByteReadable;
    use crate::code::{Chunk, Code};
    use crate::exception::Exception;
    use crate::parsing::code_parser::CodeParser;
    use crate::parsing::constant_parser::{ConstantParser, ConstantParserTable};
    use crate::parsing::raw_bytes::{RawBytes, RawBytesPointer};

    const DUMMY_CONSTANT: ConstantParser<u8> = ConstantParser {
        constant_type: 0,
        parser_fn: dummy_constant_parser,
    };

    fn dummy_constant_parser(
        bytes: &RawBytes,
        pointer: &mut RawBytesPointer,
    ) -> Result<u8, Exception> {
        Ok(bytes.read(pointer).unwrap())
    }

    fn parse(bytes: Vec<u8>) -> Code<u8> {
        let table = ConstantParserTable::parsers(&[DUMMY_CONSTANT]);
        let parser = CodeParser::new(&table);
        parser.parse(&RawBytes::from_bytes(bytes)).unwrap()
    }

    impl PartialEq for Code<u8> {
        fn eq(&self, other: &Self) -> bool {
            self.chunks == other.chunks
        }
    }

    impl PartialEq for Chunk<u8> {
        fn eq(&self, other: &Self) -> bool {
            self.constants == other.constants && self.code == other.code
        }
    }

    #[test]
    #[should_panic]
    fn test_should_panic_if_empty_code() {
        parse(vec![]);
    }

    #[test]
    fn test_should_parse_1_chunk_0_constants_0_instructions() {
        let bytes = vec![0, 0, 0];
        let code = parse(bytes);
        assert_eq!(1, code.chunks.len());
        let chunk = code.chunks.first().unwrap();
        assert_eq!(0, chunk.code.len());
        assert_eq!(0, chunk.constants.len());
    }

    #[test]
    fn test_should_parse_1_chunk_3_constants_0_instructions() {
        let bytes = vec![
            3, // 3 constants
            0, 1, // constant
            0, 2, // constant
            0, 3, // constant
            0, 0, // 0 instructions
        ];
        let code = parse(bytes);
        assert_eq!(1, code.chunks.len());
        assert_eq!(vec![1, 2, 3], code.chunks.first().unwrap().constants);
    }

    #[test]
    fn test_should_parse_1_chunk_3_constants_3_instructions() {
        let bytes = vec![
            3, // 3 constants
            0, 1, // constant
            0, 2, // constant
            0, 3, // constant
            3, 0, // 0 instructions
            4, 5, 6,
        ];
        let code = parse(bytes);
        assert_eq!(1, code.chunks.len());
        assert_eq!(vec![1, 2, 3], code.chunks.first().unwrap().constants);
        assert_eq!(vec![4, 5, 6], code.chunks.first().unwrap().code);
    }

    #[test]
    fn test_should_parse_2_chunks_1_constant_1_instruction() {
        let bytes = vec![
            // chunk #0
            1, // 1 constant
            0, 1, // constant
            1, 0,  // 1 instruction
            10, // chunk #1
            1,  // 1 constant
            0, 2, // constant
            1, 0, // 1 instruction
            11,
        ];
        let actual = parse(bytes);
        assert_eq!(2, actual.chunks.len());
        let first = actual.chunks.first().unwrap();
        let second = actual.chunks.last().unwrap();

        assert_eq!(vec![1], first.constants);
        assert_eq!(vec![10], first.code);

        assert_eq!(vec![2], second.constants);
        assert_eq!(vec![11], second.code);
    }
}
