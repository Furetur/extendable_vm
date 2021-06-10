use crate::jex::instructions::jumps::{
    CALL_INSTRUCTION, JUMP_BACKWARD, JUMP_FORWARD_IF_FALSE_INSTRUCTION, JUMP_FORWARD_INSTRUCTION,
    RETURN_INSTRUCTION,
};
use crate::jex::instructions::literal::CONSTANT_INSTRUCTION;
use crate::jex::instructions::operators::{
    ADD_INSTRUCTION, DIVIDE_INSTRUCTION, EQUAL_INSTRUCTION, FALSE_INSTRUCTION, GREATER_INSTRUCTION,
    LESS_INSTRUCTION, MULTIPLY_INSTRUCTION, NEGATE_INSTRUCTION, NOT_INSTRUCTION, NULL_INSTRUCTION,
    PRINT_INSTRUCTION, SUBTRACT_INSTRUCTION, TRUE_INSTRUCTION,
};
use crate::jex::instructions::types::JexInstruction;
use crate::jex::instructions::variable::{
    DEFINE_GLOBAL_INSTRUCTION, GET_GLOBAL_INSTRUCTION, GET_LOCAL_INSTRUCTION, POP_INSTRUCTION,
    SET_GLOBAL_INSTRUCTION, SET_LOCAL_INSTRUCTION,
};

mod jumps;
mod literal;
pub mod op_codes;
mod operators;
mod variable;

pub const JEX_INSTRUCTIONS: [&JexInstruction; 25] = [
    &NEGATE_INSTRUCTION,
    &ADD_INSTRUCTION,
    &SUBTRACT_INSTRUCTION,
    &MULTIPLY_INSTRUCTION,
    &DIVIDE_INSTRUCTION,
    &EQUAL_INSTRUCTION,
    &GREATER_INSTRUCTION,
    &LESS_INSTRUCTION,
    &NOT_INSTRUCTION,
    &PRINT_INSTRUCTION,
    &NULL_INSTRUCTION,
    &TRUE_INSTRUCTION,
    &FALSE_INSTRUCTION,
    &CONSTANT_INSTRUCTION,
    &POP_INSTRUCTION,
    &GET_LOCAL_INSTRUCTION,
    &SET_LOCAL_INSTRUCTION,
    &GET_GLOBAL_INSTRUCTION,
    &SET_GLOBAL_INSTRUCTION,
    &DEFINE_GLOBAL_INSTRUCTION,
    &JUMP_FORWARD_INSTRUCTION,
    &JUMP_FORWARD_IF_FALSE_INSTRUCTION,
    &JUMP_BACKWARD,
    &CALL_INSTRUCTION,
    &RETURN_INSTRUCTION,
];

pub mod types {
    use crate::jex::bytecode_constants::JexConstant;
    use crate::jex::jex_values::values::JexValue;
    use crate::machine::instruction::Instruction;
    use crate::machine::instruction_table::InstructionTable;

    pub type JexInstruction = Instruction<JexConstant, JexValue>;
    pub type JexInstructionTable<'a> = InstructionTable<'a, JexConstant, JexValue>;
}
