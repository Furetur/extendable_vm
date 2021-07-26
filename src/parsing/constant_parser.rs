use crate::exception::Exception;
use crate::parsing::raw_bytes::{RawBytes, RawBytesPointer};
use std::collections::HashMap;

/// Defines how bytecode constants are parsed.
///
/// Every constant in the constant pool starts with a single byte -- `constant_type`,
/// which is followed by data that encodes the value of the constant.
/// By the first byte of the constant the `ConstantParserTable` determines which `ConstantParser`
/// should be used. Then `parser_fn` should read all leading bytes and return the parsed constant.
pub struct ConstantParser<Constant> {
    pub constant_type: u8,
    pub parser_fn: ConstantParserFn<Constant>,
}

/// A function that decodes the value of the constant.
///
/// The encoded value starts at a position that is marked by `RawBytesPointer` in `RawBytes`.
type ConstantParserFn<Constant> =
    fn(&RawBytes, &mut RawBytesPointer) -> Result<Constant, Exception>;

/// A set of constant parsers
pub struct ConstantParserTable<'a, Constant> {
    parsers: HashMap<u8, &'a ConstantParser<Constant>>,
}

impl<'a, Constant> ConstantParserTable<'a, Constant> {
    pub fn parsers(parsers: &'a [ConstantParser<Constant>]) -> ConstantParserTable<'a, Constant> {
        let mut parsers_result: HashMap<u8, &'a ConstantParser<Constant>> = HashMap::new();
        for parser in parsers {
            parsers_result.insert(parser.constant_type, parser);
        }
        ConstantParserTable {
            parsers: parsers_result,
        }
    }
    pub fn get_parser(&self, constant_type: u8) -> Option<&ConstantParser<Constant>> {
        self.parsers.get(&constant_type).cloned()
    }
}
