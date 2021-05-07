use crate::run::code::TestChunk;
use jex_vm::jex::bytecode_constants::JexConstant;
use jex_vm::jex::instructions::jex_instructions;
use jex_vm::jex::instructions::types::JexInstructionTable;
use jex_vm::jex::types::JexMachine;
use jex_vm::jex::values::JexValue;
use jex_vm::machine::code::{Chunk, Code};
use jex_vm::machine::instruction_table::InstructionTable;
use jex_vm::machine::machine::Machine;

pub mod code;

pub mod run_jex {
    use crate::run::code::{TestChunk, TestInstruction};
    use jex_vm::jex::bytecode_constants::JexConstant;
    use jex_vm::machine::code::{Chunk, Code};
    use jex_vm::jex::values::{JexValue, JexFunction};
    use jex_vm::jex::instructions::types::JexInstructionTable;
    use jex_vm::jex::instructions::jex_instructions;
    use jex_vm::machine::instruction_table::InstructionTable;
    use jex_vm::jex::types::JexMachine;
    use jex_vm::machine::machine::Machine;

    pub fn run_chunks(chunks: Vec<TestChunk>) -> Option<JexValue> {
        let mut compiled_chunks: Vec<Chunk<JexConstant>> = vec![];
        for chunk in chunks {
            compiled_chunks.push(chunk.compile());
        }
        let code = Code {
            chunks: compiled_chunks,
        };
        let mut instruction_table: JexInstructionTable =
            InstructionTable::with_instructions(jex_instructions());

        let mut machine: JexMachine = Machine::new(&code, &instruction_table);
        machine.stack.push(JexValue::Function(JexFunction::Script));
        machine.stack.push_call_frame(0, 0);
        machine.run().unwrap();
        machine.stack.peek().cloned()
    }

    pub fn run_chunk(chunk: TestChunk) -> Option<JexValue> {
        run_chunks(vec![chunk])
    }

    pub fn run_instructions(instructions: Vec<TestInstruction>) -> Option<JexValue> {
        let chunk = TestChunk {
            constants: vec![],
            instructions
        };
        run_chunk(chunk)
    }
}
