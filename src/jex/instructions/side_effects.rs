use crate::machine::instruction_table::Instruction;
use crate::machine::instruction_pointer::InstructionPointer;
use crate::machine::errors::RuntimeError;
use crate::jex::values::JexValue;
use crate::jex::instructions::types::JexInstruction;
use crate::jex::types::JexMachine;

pub const SIDE_EFFECTS_INSTRUCTIONS: Vec<JexInstruction> = vec![
    Instruction {
        op_code: 10,
        name: "PRINT".to_string(),
        byte_arity: 0,
        instruction_fn: print_instruction,
    }
];

fn print_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    let value = machine.stack.pop()?;
    machine.stack.push(JexValue::Null);
    println!("{}", value.to_output_string());
    Ok(())
}
