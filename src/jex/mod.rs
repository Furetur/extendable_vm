use crate::jex::bytecode_constants::JexConstant;
use crate::jex::instructions::JEX_INSTRUCTIONS;
use crate::jex::jex_values::values::{JexFunction, JexValue};
use crate::jex::types::JexMachine;
use crate::machine::code::Code;
use crate::machine::instruction_table::InstructionTable;
use crate::machine::machine::Machine;

pub mod bytecode_constants;
pub mod constant_parsers;
pub mod instructions;
pub mod jex_values;
mod operators;
mod runtime_exceptions;
mod syntax_exceptions;

pub fn build_jex_machine(code: &Code<JexConstant>) -> JexMachine {
    let instruction_table = InstructionTable::with_instructions(&JEX_INSTRUCTIONS);
    let mut machine = Machine::new(code, instruction_table);
    machine.push_operand(JexValue::Function(JexFunction::Script));
    machine.push_frame(0, "<script>".to_string(), 0);
    machine
}

pub mod types {
    use crate::jex::bytecode_constants::JexConstant;
    use crate::jex::jex_values::values::JexValue;
    use crate::machine::machine::Machine;

    pub type JexMachine<'a> = Machine<'a, JexConstant, JexValue>;
}
