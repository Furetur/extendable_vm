use std::collections::HashMap;

use crate::instruction::Instruction;
use std::fmt::Debug;

pub struct InstructionTable<'a, Constant, Value: Debug> {
    instructions: HashMap<u8, &'a Instruction<Constant, Value>>,
}

impl<'a, Constant, Value: Debug> InstructionTable<'a, Constant, Value> {
    pub(crate) fn new() -> InstructionTable<'a, Constant, Value> {
        InstructionTable {
            instructions: HashMap::new(),
        }
    }

    pub fn instructions(
        instructions: &'a [&'a Instruction<Constant, Value>],
    ) -> InstructionTable<'a, Constant, Value> {
        let mut table: InstructionTable<Constant, Value> = InstructionTable::new();
        for instruction in instructions {
            table.register_instruction(&**instruction);
        }
        table
    }

    fn register_instruction(&mut self, instruction: &'a Instruction<Constant, Value>) {
        let prev_value = self.instructions.insert(instruction.op_code, instruction);
        if let Some(prev_instruction) = prev_value {
            panic!(
                "Instructions {} and {} have duplicate opcodes",
                prev_instruction.name, instruction.name
            );
        }
    }

    pub fn get_instruction(&self, op_code: u8) -> Option<&'a Instruction<Constant, Value>> {
        self.instructions
            .get(&op_code)
            .map(|instruction| &**instruction)
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::Instruction;
    use crate::instruction::InstructionFn::BinaryOp;
    use crate::instruction_table::InstructionTable;
    use std::ptr;
    use crate::exception::Exception;

    type Constant = i32;
    type Value = i32;

    fn add(left: i32, right: i32) -> Result<i32, Exception> {
        Ok(left + right)
    }

    fn mul(left: i32, right: i32) -> Result<i32, Exception> {
        Ok(left * right)
    }

    const ADD: Instruction<Constant, Value> = Instruction {
        op_code: 0,
        name: "ADD",
        instruction_fn: BinaryOp(add),
    };

    const MUL: Instruction<Constant, Value> = Instruction {
        op_code: 0,
        name: "MUL",
        instruction_fn: BinaryOp(mul),
    };

    #[test]
    fn initially_get_instruction_should_return_none() {
        let table: InstructionTable<Constant, Value> = InstructionTable::new();
        assert!(table.get_instruction(0).is_none());
    }

    #[test]
    fn registered_instruction_should_be_gettable() {
        let table = InstructionTable::instructions(&[&ADD]);
        assert!(ptr::eq(&ADD, table.get_instruction(0).unwrap()))
    }

    #[test]
    #[should_panic]
    fn registering_instructions_with_duplicate_opcodes_panics() {
        InstructionTable::instructions(&[&ADD, &MUL]);
    }
}
