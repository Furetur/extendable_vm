use crate::chunk::{Chunk, Instruction};
use crate::vm::VM;

mod chunk;
mod vm;

fn main() {
    let chunk = Chunk {
        constants: vec![1, 2],
        code: vec![
            Instruction::CONSTANT(0),
            Instruction::CONSTANT(1),
            Instruction::ADD,
        ],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    println!("Result {}", result)
}
