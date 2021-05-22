
use crate::machine::exceptions::types::Exception;
use crate::machine::instruction_pointer::InstructionPointer;
use crate::machine::machine::Machine;
use std::collections::HashMap;
use std::fmt::Debug;

pub struct InstructionTable<Constant, Value: Debug> {
    instructions: HashMap<u8, Instruction<Constant, Value>>,
}

pub struct Instruction<Constant, Value: Debug> {
    pub op_code: u8,
    pub name: String,
    pub byte_arity: usize,
    pub instruction_fn: InstructionFn<Constant, Value>,
}

pub type InstructionFn<Constant, Value> = fn(
    machine: &mut Machine<Constant, Value>,
    args_ip: InstructionPointer,
) -> Result<(), Exception>;

impl<Constant, Value: Debug> InstructionTable<Constant, Value> {
    pub(crate) fn new() -> InstructionTable<Constant, Value> {
        InstructionTable {
            instructions: HashMap::new(),
        }
    }

    pub fn with_instructions(
        mut instructions: Vec<Instruction<Constant, Value>>,
    ) -> InstructionTable<Constant, Value> {
        let mut table: InstructionTable<Constant, Value> = InstructionTable::new();
        while instructions.len() > 0 {
            let instruction = instructions.pop().unwrap();
            table.register_instruction(instruction);
        }
        table
    }

    pub fn register_instruction(&mut self, instruction: Instruction<Constant, Value>) {
        self.instructions.insert(instruction.op_code, instruction);
    }

    pub fn get_instruction(&self, op_code: u8) -> Option<&Instruction<Constant, Value>> {
        self.instructions.get(&op_code)
    }
}
