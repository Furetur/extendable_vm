use crate::jex::bytecode_constants::JexConstant;
use crate::jex::values::JexValue;
use crate::jex::instructions::types::JexInstruction;
use crate::jex::instructions::arithmetic::ARITHMETIC_INSTRUCTIONS;
use crate::jex::instructions::comparison::COMPARISON_INSTRUCTIONS;
use crate::jex::instructions::literal::LITERAL_INSTRUCTIONS;
use crate::jex::instructions::variable::VARIABLE_INSTRUCTIONS;
use crate::jex::instructions::side_effects::SIDE_EFFECTS_INSTRUCTIONS;
use crate::jex::instructions::jumps::JUMP_INSTRUCTIONS;
use crate::jex::instructions::logic::LOGIC_INSTRUCTIONS;

mod literal;
mod variable;
mod side_effects;
mod jumps;
mod logic;
mod comparison;
mod arithmetic;

pub const JEX_INSTRUCTIONS: Vec<JexInstruction> = {
    let mut result: Vec<JexInstruction> = vec![];
    result.append(&mut LITERAL_INSTRUCTIONS);
    result.append(&mut VARIABLE_INSTRUCTIONS);
    result.append(&mut SIDE_EFFECTS_INSTRUCTIONS);
    result.append(&mut JUMP_INSTRUCTIONS);
    result.append(&mut LOGIC_INSTRUCTIONS);
    result.append(&mut COMPARISON_INSTRUCTIONS);
    result.append(&mut ARITHMETIC_INSTRUCTIONS);
    result
};

pub mod types {
    use crate::machine::instruction_table::{Instruction, InstructionTable};
    use crate::jex::bytecode_constants::JexConstant;
    use crate::jex::values::JexValue;
    use crate::machine::machine::Machine;

    pub type JexInstruction = Instruction<JexConstant, JexValue>;
    pub type JexInstructionTable = InstructionTable<JexConstant, JexValue>;

    fn a() {

        let a = ARITHMETIC_
    }
}

