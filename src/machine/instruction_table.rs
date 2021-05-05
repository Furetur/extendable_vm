use crate::machine::errors::RuntimeError;
use crate::machine::instruction_pointer::InstructionPointer;
use crate::machine::machine::Machine;
use std::collections::HashMap;

pub struct InstructionTable<Constant, Value> {
    instructions: HashMap<u8, Instruction<Constant, Value>>,
}

pub struct Instruction<Constant, Value> {
    pub op_code: u8,
    pub name: String,
    pub byte_arity: usize,
    pub instruction_fn: InstructionFn<Constant, Value>,
}

pub type InstructionFn<Constant, Value> = fn(
    machine: &mut Machine<Constant, Value>,
    args_ip: InstructionPointer,
) -> Result<(), dyn RuntimeError>;

impl<Constant, Value> InstructionTable<Constant, Value> {
    pub(crate) fn new() -> InstructionTable<Constant, Value> {
        InstructionTable {
            instructions: HashMap::new(),
        }
    }

    pub fn register_instruction(&mut self, instruction: Instruction<Constant, Value>) {
        self.instructions[instruction.op_code] = instruction;
    }

    pub fn get_instruction(&self, op_code: u8) -> Option<&Instruction<Constant, Value>> {
        self.instructions[op_code]
    }
}
