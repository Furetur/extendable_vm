use crate::jex::bytecode_parser::BytecodeParser;
use crate::jex::bytecode_reader::BytecodeReader;
use crate::jex::instructions::types::JexInstructionTable;
use crate::jex::types::JexMachine;
use crate::machine::instruction_table::InstructionTable;
use crate::machine::machine::Machine;

pub mod bytecode_constants;
pub mod bytecode_parser;
pub mod bytecode_reader;
pub mod instructions;
pub mod values;

pub mod types {
    use crate::jex::bytecode_constants::JexConstant;
    use crate::jex::values::JexValue;
    use crate::machine::machine::Machine;

    pub type JexMachine<'a> = Machine<'a, JexConstant, JexValue>;
}
