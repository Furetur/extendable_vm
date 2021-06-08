use crate::jex::bytecode_constants::{JexConstant, JexConstantType};
use crate::machine::byte_readable::ByteReadable;
use crate::machine::exceptions::types::{Exception, ExceptionType};
use crate::machine::parsing::constant_parser::ConstantParser;
use crate::machine::parsing::parsing_exceptions::CodeEndedAt;
use crate::machine::parsing::raw_bytes::{RawBytes, RawBytesPointer};

pub type JexConstantParser = ConstantParser<JexConstant>;

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

fn parse_int_constant(
    bytes: &RawBytes,
    pointer: &mut RawBytesPointer,
) -> Result<JexConstant, Exception> {
    bytes
        .read_i32(pointer)
        .map(|int| JexConstant::Int(int))
        .ok_or(Exception::from(CodeEndedAt("i32".to_string())))
}

fn parse_string_constant(
    bytes: &RawBytes,
    pointer: &mut RawBytesPointer,
) -> Result<JexConstant, Exception> {
    let str_len = bytes
        .read_u16(pointer)
        .map(|u| usize::from(u))
        .ok_or(CodeEndedAt("u16".to_string()))?;
    let str_data = bytes
        .read_n(pointer, str_len)
        .ok_or(CodeEndedAt("string data".to_string()))?;
    let str = String::from_utf8(str_data).map_err(|err| StringConstantParsingError)?;
    Ok(JexConstant::String(str))
}

fn parse_function_constant(
    bytes: &RawBytes,
    pointer: &mut RawBytesPointer,
) -> Result<JexConstant, Exception> {
    let chunk_id = bytes
        .read(pointer)
        .map(|u| usize::from(u))
        .ok_or(CodeEndedAt("chunk_id".to_string()))?;
    Ok(JexConstant::Function { chunk_id })
}

pub struct StringConstantParsingError;

impl From<StringConstantParsingError> for Exception {
    fn from(exception: StringConstantParsingError) -> Self {
        Exception {
            exception_type: ExceptionType::Static,
            name: "StringConstantParsingError".to_string(),
            message: "Could not parse utf8 string".to_string(),
        }
    }
}
