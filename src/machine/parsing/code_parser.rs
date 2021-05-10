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
        Code { chunks }
    }
    fn parse_chunk(&self, bytes: &RawBytes, ptr: &mut RawBytesPointer) -> Chunk<Constant> {
        let mut result_constants: Vec<Constant> = vec![];
        let n_constants = bytes.read(ptr).unwrap();
        for _ in 0..n_constants {
            let constant_type = bytes.read(ptr).unwrap();
            let constant_parser = self.parsers.get_parser(constant_type);
            let constant = (constant_parser.parser_fn)(bytes, ptr);
            result_constants.push(constant);
        }
        let n_code_bytes = bytes.read_u16(ptr).unwrap();
        let code = bytes.read_n(ptr, usize::from(n_code_bytes)).unwrap();
        Chunk {
            constants: result_constants,
            code,
        }
    }
}
