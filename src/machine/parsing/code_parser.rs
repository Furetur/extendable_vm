use crate::machine::byte_readable::ByteReadable;
use crate::machine::code::{Chunk, Code};
use crate::machine::parsing::constant_parser::ConstantParserTable;
use crate::machine::parsing::raw_bytes::{RawBytes, RawBytesPointer};

pub struct CodeParser<'a, Constant> {
    parsers: &'a ConstantParserTable<'a, Constant>,
}

impl<'a, Constant> CodeParser<'a, Constant> {
    pub fn new(parsers: &'a ConstantParserTable<'a, Constant>) -> CodeParser<'a, Constant> {
        CodeParser { parsers }
    }
    pub fn parse(&self, bytes: &RawBytes) -> Code<Constant> {
        let mut chunks: Vec<Chunk<Constant>> = vec![];
        let mut ptr = RawBytesPointer::new();
        while bytes.has_next(&mut ptr) {
            chunks.push(self.parse_chunk(bytes, &mut ptr));
        }
        if chunks.len() == 0 {
            panic!("Not found any bytecode chunks");
        }
        Code { chunks }
    }
    fn parse_chunk(&self, bytes: &RawBytes, ptr: &mut RawBytesPointer) -> Chunk<Constant> {
        let mut result_constants: Vec<Constant> = vec![];
        let n_constants = bytes
            .read(ptr)
            .unwrap_or_else(|| panic!("Unexpected n_constants"));
        for _ in 0..n_constants {
            let constant_type = bytes
                .read(ptr)
                .unwrap_or_else(|| panic!("Expected constant_type"));
            let constant_parser = self.parsers.get_parser(constant_type);
            let constant = (constant_parser.parser_fn)(bytes, ptr);
            result_constants.push(constant);
        }
        let n_code_bytes = bytes
            .read_u16(ptr)
            .unwrap_or_else(|| panic!("Expected n_code_bytes"));
        let code = bytes
            .read_n(ptr, usize::from(n_code_bytes))
            .unwrap_or_else(|| panic!("Expected code"));
        Chunk {
            constants: result_constants,
            code,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::machine::byte_readable::ByteReadable;
    use crate::machine::code::{Chunk, Code};
    use crate::machine::parsing::code_parser::CodeParser;
    use crate::machine::parsing::constant_parser::{ConstantParser, ConstantParserTable};
    use crate::machine::parsing::raw_bytes::{RawBytes, RawBytesPointer};
    use std::fmt;
    use std::fmt::{Debug, Formatter};

    const DUMMY_CONSTANT: ConstantParser<u8> = ConstantParser {
        constant_type: 0,
        parser_fn: dummy_constant_parser,
    };

    fn dummy_constant_parser(bytes: &RawBytes, pointer: &mut RawBytesPointer) -> u8 {
        bytes.read(pointer).unwrap()
    }

    fn parse(bytes: Vec<u8>) -> Code<u8> {
        let table = ConstantParserTable::with_parsers(&[DUMMY_CONSTANT]);
        let parser = CodeParser::new(&table);
        parser.parse(&RawBytes::from_bytes(bytes))
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
