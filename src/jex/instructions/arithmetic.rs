use crate::jex::instructions::types::JexInstruction;
use crate::jex::types::JexMachine;
use crate::jex::values::{JexObject, JexValue};
use crate::machine::errors::MachineError;
use crate::machine::instruction_pointer::InstructionPointer;
use crate::machine::instruction_table::Instruction;
use std::rc::Rc;

pub fn arithmetic_instructions(instructions: &mut Vec<JexInstruction>) {
    let mut arithmetic_instructions = vec![
        Instruction {
            op_code: 15,
            name: "NEGATE".to_string(),
            byte_arity: 0,
            instruction_fn: negate_instruction,
        },
        Instruction {
            op_code: 16,
            name: "ADD".to_string(),
            byte_arity: 0,
            instruction_fn: add_instruction,
        },
        Instruction {
            op_code: 17,
            name: "SUBTRACT".to_string(),
            byte_arity: 0,
            instruction_fn: subtract_instruction,
        },
        Instruction {
            op_code: 18,
            name: "MULTIPLY".to_string(),
            byte_arity: 0,
            instruction_fn: multiply_instruction,
        },
        Instruction {
            op_code: 19,
            name: "DIVIDE".to_string(),
            byte_arity: 0,
            instruction_fn: divide_instruction,
        },
    ];
    instructions.append(&mut arithmetic_instructions);
}

fn negate_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), MachineError> {
    let value = machine.pop_operand()?.as_int()?;
    machine.push_operand(JexValue::Int(-value));
    Ok(())
}

fn add_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), MachineError> {
    let (left, right) = machine.pop_two_operands()?;
    let result = match (left, right) {
        (JexValue::Int(left), JexValue::Int(right)) => Ok(JexValue::Int(left + right)),
        (JexValue::Object(left), JexValue::Object(right)) => {
            let JexObject::String(left) = &*left;
            let JexObject::String(right) = &*right;
            let result = left.clone() + right;
            Ok(JexValue::Object(Rc::new(JexObject::String(result))))
        }
        (left, right) => {
            let message = format!(
                "ADD not supported for {} and {}",
                left.to_output_string(),
                right.to_output_string()
            );
            Err(MachineError(message))
        }
    }?;
    machine.push_operand(result);
    Ok(())
}

fn subtract_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), MachineError> {
    let (left, right) = machine.pop_two_operands()?;
    let (left, right) = (left.as_int()?, right.as_int()?);
    machine.push_operand(JexValue::Int(left - right));
    Ok(())
}

fn multiply_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), MachineError> {
    let (left, right) = machine.pop_two_operands()?;
    let (left, right) = (left.as_int()?, right.as_int()?);
    machine.push_operand(JexValue::Int(left * right));
    Ok(())
}

fn divide_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), MachineError> {
    let (left, right) = machine.pop_two_operands()?;
    let (left, right) = (left.as_int()?, right.as_int()?);
    machine.push_operand(JexValue::Int(left / right));
    Ok(())
}
