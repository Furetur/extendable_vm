pub mod bytecode_constants;
pub mod constant_parsers;
pub mod instructions;
mod runtime_exceptions;
mod syntax_exceptions;
pub mod values;

pub mod types {
    use crate::jex::bytecode_constants::JexConstant;
    use crate::jex::values::JexValue;
    use crate::machine::machine::Machine;

    pub type JexMachine<'a> = Machine<'a, JexConstant, JexValue>;
}
