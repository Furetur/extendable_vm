use crate::exceptions::types::Exception;
use crate::parsing::raw_bytes::{RawBytes, RawBytesPointer};
use std::collections::HashMap;

pub struct ConstantParser<Constant> {
    pub constant_type: u8,
    pub parser_fn: ConstantParserFn<Constant>,
}

type ConstantParserFn<Constant> =
    fn(&RawBytes, &mut RawBytesPointer) -> Result<Constant, Exception>;

pub struct ConstantParserTable<'a, Constant> {
    parsers: HashMap<u8, &'a ConstantParser<Constant>>,
}

impl<'a, Constant> ConstantParserTable<'a, Constant> {
    pub fn with_parsers(
        parsers: &'a [ConstantParser<Constant>],
    ) -> ConstantParserTable<'a, Constant> {
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
