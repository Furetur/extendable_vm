use crate::machine::instruction_table::Instruction;
use crate::machine::instruction_pointer::InstructionPointer;
use crate::machine::errors::RuntimeError;
use crate::jex::values::JexValue;
use crate::jex::instructions::types::JexInstruction;
use crate::jex::types::JexMachine;


pub const COMPARISON_INSTRUCTIONS: Vec<JexInstruction> = vec![
    Instruction {
        op_code: 12,
        name: "EQUAL".to_string(),
        byte_arity: 0,
        instruction_fn: equal_instruction
    },
    Instruction {
        op_code: 13,
        name: "GREATER".to_string(),
        byte_arity: 0,
        instruction_fn: greater_instruction
    },
    Instruction {
        op_code: 14,
        name: "LESS".to_string(),
        byte_arity: 0,
        instruction_fn: less_instruction,
    }
];


fn equal_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    let (left, right) = machine.stack.pop_two_operands()?;
    let result = left == right;
    machine.stack.push(JexValue::Bool(result));
    Ok(())
}

fn greater_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    let (left, right) = machine.stack.pop_two_operands()?;
    let (left, right) = (left.as_int()?, right.as_int()?);
    machine.stack.push(JexValue::Bool(left > right));
    Ok(())
}

fn less_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    let (left, right) = machine.stack.pop_two_operands()?;
    let (left, right) = (left.as_int()?, right.as_int()?);
    machine.stack.push(JexValue::Bool(left < right));
    Ok(())
}