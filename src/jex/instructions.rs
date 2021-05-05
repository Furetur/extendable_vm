use crate::jex::bytecode_constants::JexConstant;
use crate::jex::values::{JexFunction, JexObject, JexValue};
use crate::machine::errors::{RuntimeError, TypeError};
use crate::machine::instruction_pointer::InstructionPointer;
use crate::machine::machine::Machine;
use std::rc::Rc;
use std::ops::Add;
use crate::machine::instruction_table::{InstructionTable, Instruction};

pub type JexMachine<'a> = Machine<'a, JexConstant, JexValue>;
pub type JexInstruction = Instruction<JexConstant, JexValue>;
pub type JexInstructionTable = InstructionTable<JexConstant, JexValue>;

fn get_jex_instruction_table() -> JexInstructionTable {
    let mut table: JexInstructionTable = InstructionTable::new();

    let instructions = vec![
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
        },
        Instruction {
            op_code: 4,
            name: "POP".to_string(),
            byte_arity: 0,
            instruction_fn: pop_instruction
        },
        Instruction {
            op_code: 5,
            name: "GET_LOCAL".to_string(),
            byte_arity: 1,
            instruction_fn: get_local_instruction
        },
        Instruction {
            op_code: 6,
            name: "SET_LOCAL".to_string(),
            byte_arity: 1,
            instruction_fn: set_local_instruction
        },
        Instruction {
            op_code: 7,
            name: "GET_GLOBAL".to_string(),
            byte_arity: 1,
            instruction_fn: pop_instruction
        },
        Instruction {
            op_code: 8,
            name: "DEFINE_GLOBAL".to_string(),
            byte_arity: 1,
            instruction_fn: define_global_instruction
        },
        Instruction {
            op_code: 9,
            name: "SET_GLOBAL".to_string(),
            byte_arity: 1,
            instruction_fn: set_global_instruction
        },
        Instruction {
            op_code: 10,
            name: "PRINT".to_string(),
            byte_arity: 0,
            instruction_fn: print_instruction,
        },
        Instruction {
            op_code: 11,
            name: "NOT".to_string(),
            byte_arity: 0,
            instruction_fn: not_instruction,
        },
        Instruction {
            op_code: 12,
            name: "EQUAL".to_string(),
            byte_arity: 0,
            instruction_fn: equal_instruction
        },
        Instruction {
            op_code: 13,
            name: "GREATER".to_string(),
            byte_arity: 0,
            instruction_fn: greater_instruction
        },
        Instruction {
            op_code: 14,
            name: "LESS".to_string(),
            byte_arity: 0,
            instruction_fn: less_instruction,
        },
        Instruction {
            op_code: 15,
            name: "NEGATE".to_string(),
            byte_arity: 0,
            instruction_fn: negate_instruction
        },
        Instruction {
            op_code: 16,
            name: "ADD".to_string(),
            byte_arity: 0,
            instruction_fn: add_instruction
        },
        Instruction {
            op_code: 17,
            name: "SUBTRACT".to_string(),
            byte_arity: 0,
            instruction_fn: subtract_instruction
        },
        Instruction {
            op_code: 18,
            name: "MULTIPLY".to_string(),
            byte_arity: 0,
            instruction_fn: multiply_instruction
        },
        Instruction {
            op_code: 19,
            name: "DIVIDE".to_string(),
            byte_arity: 0,
            instruction_fn: divide_instruction
        },
        Instruction {
            op_code: 20,
            name: "JUMP_FORWARD".to_string(),
            byte_arity: 1,
            instruction_fn: jump_forward_instruction
        },
        Instruction {
            op_code: 21,
            name: "JUMP_FORWARD_IF_FALSE".to_string(),
            byte_arity: 1,
            instruction_fn: jump_forward_if_false_instruction
        },
        Instruction {
            op_code: 22,
            name: "JUMP_BACKWARD".to_string(),
            byte_arity: 1,
            instruction_fn: jump_backward_instruction
        },
        Instruction {
            op_code: 20,
            name: "CALL".to_string(),
            byte_arity: 1,
            instruction_fn: call_instruction
        }
    ];
}

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

fn pop_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    machine.stack.pop()?;
    Ok(())
}

fn get_local_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
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
) -> Result<(), impl RuntimeError> {
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
) -> Result<(), impl RuntimeError> {
    let identifier_const_id = machine
        .code
        .read_for(&mut arguments_ip, "GET_GLOBAL byte operand")?;
    let identifier = machine
        .code
        .get_constant(arguments_ip.chunk_id, usize::from(identifier_const_id))?;
    let identifier_string = identifier.as_string()?;
    let value = machine.globals[identifier].clone();
    machine.stack.push(value);
    Ok(())
}

fn define_global_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    let identifier_const_id = machine
        .code
        .read_for(&mut arguments_ip, "GET_GLOBAL byte operand")?;
    let identifier = machine
        .code
        .get_constant(arguments_ip.chunk_id, usize::from(identifier_const_id))?;
    let value = machine.stack.pop()?;
    machine.globals[identifier] = value;
    Ok(())
}

fn set_global_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    define_global_instruction(machine, arguments_ip)
}

fn print_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    let value = machine.stack.pop()?;
    machine.stack.push(JexValue::Null);
    println!("{}", value.to_output_string());
    Ok(())
}

fn not_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    let value = machine.stack.pop()?.as_bool()?;
    machine.stack.push(JexValue::Bool(!value));
    Ok(())
}

fn equal_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    let (left, right) = machine.stack.pop_two_operands()?;
    let result = left == right;
    machine.stack.push(JexValue::Bool(result));
    Ok(())
}

fn greater_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    let (left, right) = machine.stack.pop_two_operands()?;
    let (left, right) = (left.as_int()?, right.as_int()?);
    machine.stack.push(JexValue::Bool(left > right));
    Ok(())
}

fn less_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    let (left, right) = machine.stack.pop_two_operands()?;
    let (left, right) = (left.as_int()?, right.as_int()?);
    machine.stack.push(JexValue::Bool(left < right));
    Ok(())
}

fn negate_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    let value = machine.stack.pop()?.as_int()?;
    machine.stack.push(JexValue::Int(-value));
    Ok(())
}

fn add_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    let (left, right) = machine.stack.pop_two_operands()?;
    let result = match (left, right) {
        (JexValue::Int(left), JexValue::Int(right)) => Ok(JexValue::Int(left + right)),
        (JexValue::Object(left), JexValue::Object(right)) => {
            let JexObject::String(left) = &*left;
            let JexObject::String(right) = &*right;
            let result = left.clone() + right;
            Ok(JexValue::Object(Rc::new(JexObject::String(result))))
        }
        _ => {
            let message = format!(
                "ADD not supported for {} and {}",
                left.to_output_string(),
                right.to_output_string()
            );
            Err(TypeError(message))
        }
    }?;
    machine.stack.push(result);
    Ok(())
}

fn subtract_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    let (left, right) = machine.stack.pop_two_operands()?;
    let (left, right) = (left.as_int()?, right.as_int()?);
    machine.stack.push(JexValue::Int(left - right));
    Ok(())
}

fn multiply_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    let (left, right) = machine.stack.pop_two_operands()?;
    let (left, right) = (left.as_int()?, right.as_int()?);
    machine.stack.push(JexValue::Int(left * right));
    Ok(())
}

fn divide_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    let (left, right) = machine.stack.pop_two_operands()?;
    let (left, right) = (left.as_int()?, right.as_int()?);
    machine.stack.push(JexValue::Int(left / right));
    Ok(())
}

fn jump_forward_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    let offset = machine
        .code
        .read_for(&mut arguments_ip, "JUMP_FORWARD operand")?;
    machine
        .stack
        .current_ip()?
        .jump_forward(usize::from(offset));
    Ok(())
}

fn jump_forward_if_false_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    let offset = machine
        .code
        .read_for(&mut arguments_ip, "JUMP_FORWARD_IF_FALSE operand")?;
    let value = machine.stack.peek().unwrap().as_bool()?;
    if !value {
        machine
            .stack
            .current_ip()?
            .jump_forward(usize::from(offset));
    }
    Ok(())
}

fn jump_backward_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    let offset = machine
        .code
        .read_for(&mut arguments_ip, "JUMP_FORWARD operand")?;
    machine
        .stack
        .current_ip()?
        .jump_backward(usize::from(offset));
    Ok(())
}

fn call_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    let arity = machine.code.read_for(&mut arguments_ip, "CALL operand")?;
    let arity = usize::from(arity);
    let function = machine
        .stack
        .peek_from_top(usize::from(arity))?
        .as_function()?;
    if let JexFunction::Function {
        chunk_id,
        arity: actual_arity,
        ..
    } = function
    {
        if arity == *actual_arity {
            machine.stack.push_call_frame(*chunk_id, arity);
            Ok(())
        } else {
            Err(TypeError(format!(
                "Function {} has {} parameters but received {}",
                function.to_output_string(),
                actual_arity,
                arity
            )))
        }
    } else {
        Err(TypeError(format!(
            "Cannot call {}",
            function.to_output_string()
        )))
    }
}

fn return_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    let return_value = machine.stack.pop()?;
    machine.stack.discard_call_frame()?;
    machine.stack.push(return_value);
    Ok(())
}
