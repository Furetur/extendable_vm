use crate::machine::instruction_table::Instruction;
use crate::machine::instruction_pointer::InstructionPointer;
use crate::machine::errors::MachineError;
use crate::jex::values::JexValue;
use crate::jex::instructions::types::JexInstruction;
use crate::jex::types::JexMachine;

pub fn logic_instructions(instructions: &mut Vec<JexInstruction>) {
    let mut logic_instructions = vec![
        Instruction {
            op_code: 11,
            name: "NOT".to_string(),
            byte_arity: 0,
            instruction_fn: not_instruction,
        }
    ];
    instructions.append(&mut logic_instructions);
}

fn not_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), MachineError> {
    let value = machine.stack.pop()?.as_bool()?;
    machine.stack.push(JexValue::Bool(!value));
    Ok(())
}
