use crate::chunk::{Chunk, Instruction, ChunkConstant};
use crate::vm::VM;

mod chunk;
mod vm;
mod jexvalues;

fn main() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::INT(1), ChunkConstant::from_str("string")],
        code: vec![
            Instruction::CONSTANT(1),
            Instruction::CONSTANT(1),
            Instruction::ADD,
        ],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    println!("Result {:?}", result.unwrap());
}
