use crate::machine::parsing::raw_bytes::{RawBytes, RawBytesPointer};
use std::collections::HashMap;

pub struct ConstantParser<Constant> {
    pub constant_type: u8,
    pub parser_fn: ConstantParserFn<Constant>,
}

type ConstantParserFn<Constant> = fn(&RawBytes, &mut RawBytesPointer) -> Constant;

pub struct ConstantParserTable<'a, Constant> {
    parsers: HashMap<u8, &'a ConstantParser<Constant>>,
}

impl<'a, Constant> ConstantParserTable<'a, Constant> {
    pub fn register_parser(&mut self, parser: &'static ConstantParser<Constant>) {
        self.parsers.insert(parser.constant_type, parser);
    }
    pub fn get_parser(&self, constant_type: u8) -> &ConstantParser<Constant> {
        self.parsers.get(&constant_type).unwrap()
    }
}
