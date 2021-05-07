use crate::jex::bytecode_constants::JexConstant;
use crate::jex::instructions::arithmetic::arithmetic_instructions;
use crate::jex::instructions::comparison::comparison_instructions;
use crate::jex::instructions::jumps::jump_instructions;
use crate::jex::instructions::literal::literal_instructions;
use crate::jex::instructions::logic::logic_instructions;
use crate::jex::instructions::side_effects::side_effects_instructions;
use crate::jex::instructions::types::JexInstruction;
use crate::jex::instructions::variable::variable_instructions;
use crate::jex::values::JexValue;

mod arithmetic;
mod comparison;
mod jumps;
mod literal;
mod logic;
pub mod op_codes;
mod side_effects;
mod variable;

pub fn jex_instructions() -> Vec<JexInstruction> {
    let mut instructions: Vec<JexInstruction> = vec![];
    arithmetic_instructions(&mut instructions);
    comparison_instructions(&mut instructions);
    jump_instructions(&mut instructions);
    literal_instructions(&mut instructions);
    logic_instructions(&mut instructions);
    side_effects_instructions(&mut instructions);
    variable_instructions(&mut instructions);
    instructions
}

pub mod types {
    use crate::jex::bytecode_constants::JexConstant;
    use crate::jex::values::JexValue;
    use crate::machine::instruction_table::{Instruction, InstructionTable};
    use crate::machine::machine::Machine;

    pub type JexInstruction = Instruction<JexConstant, JexValue>;
    pub type JexInstructionTable = InstructionTable<JexConstant, JexValue>;
}
