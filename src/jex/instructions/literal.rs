use crate::jex::instructions::op_codes::JexOpCode;
use crate::jex::instructions::types::JexInstruction;
use crate::jex::jex_values::values::{JexNull, JexValue};
use crate::jex::runtime_exceptions::ExpectedInstructionArgument;
use crate::jex::types::JexMachine;
use crate::machine::byte_readable::ByteReadable;
use crate::machine::exceptions::types::Exception;
use crate::machine::instruction::Instruction;
use crate::machine::instruction::InstructionFn::Raw;
use crate::machine::instruction_pointer::InstructionPointer;

pub const CONSTANT_INSTRUCTION: JexInstruction = Instruction {
    op_code: JexOpCode::Constant as u8,
    name: "CONSTANT",
    instruction_fn: Raw {
        byte_arity: 1,
        instruction_fn: constant_instruction,
    },
};

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
