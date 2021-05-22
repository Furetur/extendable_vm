use crate::jex::instructions::types::JexInstruction;
use crate::jex::runtime_exceptions::{ExpectedInstructionArgument, TypeException};
use crate::jex::types::JexMachine;
use crate::machine::byte_readable::ByteReadable;
use crate::machine::exceptions::types::Exception;
use crate::machine::instruction_pointer::InstructionPointer;
use crate::machine::instruction_table::Instruction;

pub fn variable_instructions(instructions: &mut Vec<JexInstruction>) {
    let mut variable_instructions = vec![
        Instruction {
            op_code: 4,
            name: "POP".to_string(),
            byte_arity: 0,
            instruction_fn: pop_instruction,
        },
        Instruction {
            op_code: 5,
            name: "GET_LOCAL".to_string(),
            byte_arity: 1,
            instruction_fn: get_local_instruction,
        },
        Instruction {
            op_code: 6,
            name: "SET_LOCAL".to_string(),
            byte_arity: 1,
            instruction_fn: set_local_instruction,
        },
        Instruction {
            op_code: 7,
            name: "GET_GLOBAL".to_string(),
            byte_arity: 1,
            instruction_fn: get_global_instruction,
        },
        Instruction {
            op_code: 8,
            name: "DEFINE_GLOBAL".to_string(),
            byte_arity: 1,
            instruction_fn: define_global_instruction,
        },
        Instruction {
            op_code: 9,
            name: "SET_GLOBAL".to_string(),
            byte_arity: 1,
            instruction_fn: set_global_instruction,
        },
    ];

    instructions.append(&mut variable_instructions);
}

fn pop_instruction(
    machine: &mut JexMachine,
    mut _args: InstructionPointer,
) -> Result<(), Exception> {
    machine.pop_operand()?;
    Ok(())
}

fn get_local_instruction(
    machine: &mut JexMachine,
    mut args: InstructionPointer,
) -> Result<(), Exception> {
    let relative_slot = machine.read(&mut args).ok_or(ExpectedInstructionArgument)?;
    let last_frame_start = machine.peek_frame()?.start_slot;
    let absolute_slot = last_frame_start + usize::from(relative_slot);
    let value = machine.get_operand(absolute_slot)?.clone();
    machine.push_operand(value);
    Ok(())
}

fn set_local_instruction(
    machine: &mut JexMachine,
    mut args: InstructionPointer,
) -> Result<(), Exception> {
    let relative_slot = machine.read(&mut args).ok_or(ExpectedInstructionArgument)?;
    let absolute_slot = machine.peek_frame()?.start_slot + usize::from(relative_slot);
    let value = machine.pop_operand()?;
    machine.set_operand(absolute_slot, value)?;
    Ok(())
}

fn get_global_instruction(
    machine: &mut JexMachine,
    mut args: InstructionPointer,
) -> Result<(), Exception> {
    let identifier_const_id = machine.read(&mut args).ok_or(ExpectedInstructionArgument)?;
    let identifier = machine
        .code
        .get_constant(args.chunk_id, usize::from(identifier_const_id))?;
    let identifier_string = identifier.as_string()?;
    let value = machine.globals.get(&identifier_string).cloned();
    if let Some(value) = value {
        machine.push_operand(value);
        Ok(())
    } else {
        Err(Exception::from(TypeException(format!(
            "Global with identifier {} not found",
            identifier_string
        ))))
    }
}

fn define_global_instruction(
    machine: &mut JexMachine,
    mut args: InstructionPointer,
) -> Result<(), Exception> {
    let identifier_const_id = machine.read(&mut args).ok_or(ExpectedInstructionArgument)?;
    let identifier = machine
        .code
        .get_constant(args.chunk_id, usize::from(identifier_const_id))?
        .as_string()?;
    let value = machine.pop_operand()?;
    machine.globals.insert(identifier, value);
    Ok(())
}

fn set_global_instruction(
    machine: &mut JexMachine,
    args: InstructionPointer,
) -> Result<(), Exception> {
    define_global_instruction(machine, args)
}
