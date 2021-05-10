use crate::jex::bytecode_constants::{JexConstant, JexConstantType};
use crate::machine::byte_readable::ByteReadable;
use crate::machine::parsing::constant_parser::ConstantParser;
use crate::machine::parsing::raw_bytes::{RawBytes, RawBytesPointer};

pub type JexConstantParser = ConstantParser<JexConstant>;

pub fn jex_constant_parsers(parsers: &mut Vec<JexConstantParser>) {
    parsers.push(ConstantParser {
        constant_type: JexConstantType::Int as u8,
        parser_fn: parse_int_constant,
    });
    parsers.push(ConstantParser {
        constant_type: JexConstantType::String as u8,
        parser_fn: parse_string_constant,
    });
    parsers.push(ConstantParser {
        constant_type: JexConstantType::Function as u8,
        parser_fn: parse_function_constant,
    })
}

pub const JEX_CONSTANT_PARSERS: [JexConstantParser; 3] = [
    ConstantParser {
        constant_type: JexConstantType::Int as u8,
        parser_fn: parse_int_constant,
    },
    ConstantParser {
        constant_type: JexConstantType::String as u8,
        parser_fn: parse_string_constant,
    },
    ConstantParser {
        constant_type: JexConstantType::Function as u8,
        parser_fn: parse_function_constant,
    },
];

fn parse_int_constant(bytes: &RawBytes, pointer: &mut RawBytesPointer) -> JexConstant {
    let int = bytes.read_i32(pointer).unwrap();
    JexConstant::Int(int)
}

fn parse_string_constant(bytes: &RawBytes, pointer: &mut RawBytesPointer) -> JexConstant {
    let str_len = usize::from(bytes.read_u16(pointer).unwrap());
    let str_data = bytes.read_n(pointer, str_len).unwrap();
    JexConstant::String(String::from_utf8(str_data).unwrap())
}

fn parse_function_constant(bytes: &RawBytes, pointer: &mut RawBytesPointer) -> JexConstant {
    let chunk_id = usize::from(bytes.read(pointer).unwrap());
    JexConstant::Function { chunk_id }
}
