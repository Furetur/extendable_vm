use crate::chunk::{Chunk, Instruction, ChunkConstant};
use crate::vm::VM;

mod chunk;
mod vm;
mod jexvalues;
mod jexobject;
mod string_interner;
mod operators;

fn main() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::INT(1), ChunkConstant::from_str("string"), ChunkConstant::from_str("string")],
        code: vec![
            Instruction::CONSTANT(1),
            Instruction::CONSTANT(1),
            Instruction::ADD,
            Instruction::PRINT,
            Instruction::CONSTANT(1),
            Instruction::CONSTANT(2),
            Instruction::EQUAL,
            Instruction::PRINT
        ],
    };
    let mut vm = VM::new();
    vm.run(&chunk);
}
