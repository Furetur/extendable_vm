pub mod bytecode_constants;
pub mod constant_parsers;
pub mod instructions;
pub mod jex_values;
mod runtime_exceptions;
mod syntax_exceptions;

pub mod types {
    use crate::jex::bytecode_constants::JexConstant;
    use crate::jex::jex_values::values::JexValue;
    use crate::machine::machine::Machine;

    pub type JexMachine<'a> = Machine<'a, JexConstant, JexValue>;
}
