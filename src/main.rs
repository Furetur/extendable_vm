use crate::chunk::{Chunk, ChunkConstant, Instruction};
use crate::vm::VM;

mod chunk;
mod jexobject;
mod jexvalues;
mod operators;
mod string_interner;
mod vm;

fn main() {
    let chunk = Chunk {
        constants: vec![
            ChunkConstant::INT(1),
            ChunkConstant::from_str("string"),
            ChunkConstant::from_str("string"),
        ],
        code: vec![
            Instruction::Constant(1),
            Instruction::Constant(1),
            Instruction::Add,
            Instruction::Print,
            Instruction::Constant(1),
            Instruction::Constant(2),
            Instruction::Equal,
            Instruction::Print,
        ],
    };
    let mut vm = VM::new();
    vm.run(&chunk);
}
