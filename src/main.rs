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
            Instruction::Constant(1),
            Instruction::Constant(1),
            Instruction::Add,
            Instruction::Print,
            Instruction::Constant(1),
            Instruction::Constant(2),
            Instruction::Equal,
            Instruction::Print
        ],
    };
    let mut vm = VM::new();
    vm.run(&chunk);
}
