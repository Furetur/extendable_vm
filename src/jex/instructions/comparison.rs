use crate::jex::instructions::types::JexInstruction;
use crate::jex::types::JexMachine;
use crate::jex::values::JexValue;
use crate::machine::errors::MachineError;
use crate::machine::instruction_pointer::InstructionPointer;
use crate::machine::instruction_table::Instruction;

pub fn comparison_instructions(instructions: &mut Vec<JexInstruction>) {
    let mut comparison_instructions = vec![
        Instruction {
            op_code: 12,
            name: "EQUAL".to_string(),
            byte_arity: 0,
            instruction_fn: equal_instruction,
        },
        Instruction {
            op_code: 13,
            name: "GREATER".to_string(),
            byte_arity: 0,
            instruction_fn: greater_instruction,
        },
        Instruction {
            op_code: 14,
            name: "LESS".to_string(),
            byte_arity: 0,
            instruction_fn: less_instruction,
        },
    ];
    instructions.append(&mut comparison_instructions);
}

fn equal_instruction(
    machine: &mut JexMachine,
    mut _args: InstructionPointer,
) -> Result<(), MachineError> {
    let (left, right) = machine.pop_two_operands()?;
    let result = left == right;
    machine.push_operand(JexValue::Bool(result));
    Ok(())
}

fn greater_instruction(
    machine: &mut JexMachine,
    mut _args: InstructionPointer,
) -> Result<(), MachineError> {
    let (left, right) = machine.pop_two_operands()?;
    let (left, right) = (left.as_int()?, right.as_int()?);
    machine.push_operand(JexValue::Bool(left > right));
    Ok(())
}

fn less_instruction(
    machine: &mut JexMachine,
    mut _args: InstructionPointer,
) -> Result<(), MachineError> {
    let (left, right) = machine.pop_two_operands()?;
    let (left, right) = (left.as_int()?, right.as_int()?);
    machine.push_operand(JexValue::Bool(left < right));
    Ok(())
}
