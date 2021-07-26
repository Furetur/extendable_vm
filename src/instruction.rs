use crate::exception::Exception;
use crate::{InstructionPointer, Machine};
use std::fmt::Debug;

/// Describes one type of instructions that the VM supports.
///
/// The instruction must have a unique id (`op_code`), a `name` for debugging
/// and an `instruction_fn` which implements the logic of the instruction.
#[derive(Clone)]
pub struct Instruction<Constant, Value: Debug> {
    pub op_code: u8,
    pub name: &'static str,
    pub instruction_fn: InstructionFn<Constant, Value>,
}

/// A function that implements a certain instruction type.
#[derive(Clone)]
pub enum InstructionFn<Constant, Value: Debug> {
    Raw {
        byte_arity: usize,
        instruction_fn: RawInstructionFn<Constant, Value>,
    },
    Const(fn() -> Value),
    UnaryOp(fn(value: Value) -> Result<Value, Exception>),
    BinaryOp(fn(left: Value, right: Value) -> Result<Value, Exception>),
}

pub type RawInstructionFn<Constant, Value> = fn(
    machine: &mut Machine<Constant, Value>,
    args_ip: InstructionPointer,
) -> Result<(), Exception>;

impl<Constant, Value: Debug> InstructionFn<Constant, Value> {
    pub fn byte_arity(&self) -> usize {
        if let InstructionFn::Raw { byte_arity, .. } = self {
            *byte_arity
        } else {
            0
        }
    }
    pub fn run(
        &self,
        machine: &mut Machine<Constant, Value>,
        args_ip: InstructionPointer,
    ) -> Result<(), Exception> {
        match self {
            InstructionFn::Raw { instruction_fn, .. } => {
                instruction_fn(machine, args_ip)?;
            }
            InstructionFn::Const(get_value) => {
                machine.push_operand(get_value());
            }
            InstructionFn::UnaryOp(operator) => {
                let operand = machine.pop_operand()?;
                let result = (*operator)(operand)?;
                machine.push_operand(result);
            }
            InstructionFn::BinaryOp(operator) => {
                let (left, right) = machine.pop_two_operands()?;
                let result = (*operator)(left, right)?;
                machine.push_operand(result);
            }
        };
        Ok(())
    }
}
