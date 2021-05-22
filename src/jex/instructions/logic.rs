use crate::jex::instructions::types::JexInstruction;
use crate::jex::types::JexMachine;
use crate::jex::values::JexValue;
use crate::machine::exceptions::types::Exception;
use crate::machine::instruction_pointer::InstructionPointer;
use crate::machine::instruction_table::Instruction;

pub fn logic_instructions(instructions: &mut Vec<JexInstruction>) {
    let mut logic_instructions = vec![Instruction {
        op_code: 11,
        name: "NOT".to_string(),
        byte_arity: 0,
        instruction_fn: not_instruction,
    }];
    instructions.append(&mut logic_instructions);
}

fn not_instruction(
    machine: &mut JexMachine,
    mut _args: InstructionPointer,
) -> Result<(), Exception> {
    let value = machine.pop_operand()?.as_bool()?;
    machine.push_operand(JexValue::Bool(!value));
    Ok(())
}
