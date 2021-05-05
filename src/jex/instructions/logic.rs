use crate::machine::instruction_table::Instruction;
use crate::machine::instruction_pointer::InstructionPointer;
use crate::machine::errors::RuntimeError;
use crate::jex::values::JexValue;
use crate::jex::instructions::types::JexInstruction;
use crate::jex::types::JexMachine;

pub const LOGIC_INSTRUCTIONS: Vec<JexInstruction> = vec![
    Instruction {
        op_code: 11,
        name: "NOT".to_string(),
        byte_arity: 0,
        instruction_fn: not_instruction,
    }
];

fn not_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    let value = machine.stack.pop()?.as_bool()?;
    machine.stack.push(JexValue::Bool(!value));
    Ok(())
}
