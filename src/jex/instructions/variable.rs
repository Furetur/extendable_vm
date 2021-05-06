use crate::jex::instructions::types::JexInstruction;
use crate::jex::types::JexMachine;
use crate::machine::errors::MachineError;
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
            instruction_fn: pop_instruction,
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
    mut arguments_ip: InstructionPointer,
) -> Result<(), MachineError> {
    machine.stack.pop()?;
    Ok(())
}

fn get_local_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), MachineError> {
    let relative_slot = machine
        .code
        .read_for(&mut arguments_ip, "GET_LOCAL argument")?;
    let value = machine.stack.get_local(usize::from(relative_slot))?;
    machine.stack.push(value.clone());
    Ok(())
}

fn set_local_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), MachineError> {
    let relative_slot = machine
        .code
        .read_for(&mut arguments_ip, "SET_LOCAL argument")?;
    let value = machine.stack.pop()?;
    machine.stack.set_local(usize::from(relative_slot), value);
    Ok(())
}

fn get_global_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), MachineError> {
    let identifier_const_id = machine
        .code
        .read_for(&mut arguments_ip, "GET_GLOBAL byte operand")?;
    let identifier = machine
        .code
        .get_constant(arguments_ip.chunk_id, usize::from(identifier_const_id))?;
    let identifier_string = identifier.as_string()?;
    let value = machine.globals.get(&identifier_string).cloned();
    if let Some(value) = value {
        machine.stack.push(value);
        Ok(())
    } else {
        Err(MachineError(format!(
            "Global with identifier {} not found",
            identifier_string
        )))
    }
}

fn define_global_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), MachineError> {
    let identifier_const_id = machine
        .code
        .read_for(&mut arguments_ip, "GET_GLOBAL byte operand")?;
    let identifier = machine
        .code
        .get_constant(arguments_ip.chunk_id, usize::from(identifier_const_id))?
        .as_string()?;
    let value = machine.stack.pop()?;
    machine.globals.insert(identifier, value);
    Ok(())
}

fn set_global_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), MachineError> {
    define_global_instruction(machine, arguments_ip)
}
