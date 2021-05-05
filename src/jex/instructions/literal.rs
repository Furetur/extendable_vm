use crate::machine::instruction_table::Instruction;
use crate::machine::instruction_pointer::InstructionPointer;
use crate::machine::errors::RuntimeError;
use crate::jex::values::JexValue;
use crate::jex::instructions::types::JexInstruction;
use crate::jex::types::JexMachine;

pub const LITERAL_INSTRUCTIONS: Vec<JexInstruction> = vec![
    Instruction {
        op_code: 0,
        name: "CONSTANT".to_string(),
        byte_arity: 1,
        instruction_fn: constant_instruction
    },
    Instruction {
        op_code: 1,
        name: "NULL".to_string(),
        byte_arity: 0,
        instruction_fn: null_instruction
    },
    Instruction {
        op_code: 2,
        name: "TRUE".to_string(),
        byte_arity: 0,
        instruction_fn: true_instruction
    },
    Instruction {
        op_code: 3,
        name: "FALSE".to_string(),
        byte_arity: 0,
        instruction_fn: false_instruction
    }
];


fn constant_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    let constant_id = machine
        .code
        .read_for(&mut arguments_ip, "CONSTANT argument")?;
    let constant = machine
        .code
        .get_constant(arguments_ip.chunk_id, usize::from(constant_id))?;
    machine.stack.push(constant.to_value());
    Ok(())
}

fn null_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    machine.stack.push(JexValue::Null);
    Ok(())
}

fn true_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    machine.stack.push(JexValue::Bool(true));
    Ok(())
}

fn false_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    machine.stack.push(JexValue::Bool(false));
    Ok(())
}