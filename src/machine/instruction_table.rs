use std::collections::HashMap;
use std::fmt::Debug;

use crate::machine::exceptions::types::Exception;
use crate::machine::instruction::Instruction;
use crate::machine::instruction_pointer::InstructionPointer;
use crate::machine::machine::Machine;

pub struct InstructionTable<'a, Constant, Value> {
    instructions: HashMap<u8, &'a Instruction<Constant, Value>>,
}

impl<'a, Constant, Value> InstructionTable<'a, Constant, Value> {
    pub(crate) fn new() -> InstructionTable<'a, Constant, Value> {
        InstructionTable {
            instructions: HashMap::new(),
        }
    }

    pub fn with_instructions(
        mut instructions: Vec<&'a Instruction<Constant, Value>>,
    ) -> InstructionTable<'a, Constant, Value> {
        let mut table: InstructionTable<Constant, Value> = InstructionTable::new();
        while instructions.len() > 0 {
            let instruction = instructions.pop().unwrap();
            table.register_instruction(instruction);
        }
        table
    }

    pub fn register_instruction(&mut self, instruction: &'a Instruction<Constant, Value>) {
        self.instructions.insert(instruction.op_code, instruction);
    }

    pub fn get_instruction(&self, op_code: u8) -> Option<&'a Instruction<Constant, Value>> {
        self.instructions
            .get(&op_code)
            .map(|instruction| &**instruction)
    }
}
