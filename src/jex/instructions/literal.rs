use crate::jex::instructions::types::JexInstruction;
use crate::jex::runtime_exceptions::ExpectedInstructionArgument;
use crate::jex::types::JexMachine;
use crate::jex::values::JexValue;
use crate::machine::byte_readable::ByteReadable;
use crate::machine::exceptions::types::Exception;
use crate::machine::instruction_pointer::InstructionPointer;
use crate::machine::instruction_table::Instruction;

pub fn literal_instructions(instructions: &mut Vec<JexInstruction>) {
    let mut literal_instructions = vec![
        Instruction {
            op_code: 0,
            name: "CONSTANT".to_string(),
            byte_arity: 1,
            instruction_fn: constant_instruction,
        },
        Instruction {
            op_code: 1,
            name: "NULL".to_string(),
            byte_arity: 0,
            instruction_fn: null_instruction,
        },
        Instruction {
            op_code: 2,
            name: "TRUE".to_string(),
            byte_arity: 0,
            instruction_fn: true_instruction,
        },
        Instruction {
            op_code: 3,
            name: "FALSE".to_string(),
            byte_arity: 0,
            instruction_fn: false_instruction,
        },
    ];

    instructions.append(&mut literal_instructions);
}

fn constant_instruction(
    machine: &mut JexMachine,
    mut args: InstructionPointer,
) -> Result<(), Exception> {
    let constant_id = machine.read(&mut args).ok_or(ExpectedInstructionArgument)?;
    let constant = machine
        .code
        .get_constant(args.chunk_id, usize::from(constant_id))?;
    machine.push_operand(constant.to_value(machine)?);
    Ok(())
}

fn null_instruction(
    machine: &mut JexMachine,
    mut _args: InstructionPointer,
) -> Result<(), Exception> {
    machine.push_operand(JexValue::Null);
    Ok(())
}

fn true_instruction(
    machine: &mut JexMachine,
    mut _args: InstructionPointer,
) -> Result<(), Exception> {
    machine.push_operand(JexValue::Bool(true));
    Ok(())
}

fn false_instruction(
    machine: &mut JexMachine,
    mut _args: InstructionPointer,
) -> Result<(), Exception> {
    machine.push_operand(JexValue::Bool(false));
    Ok(())
}
