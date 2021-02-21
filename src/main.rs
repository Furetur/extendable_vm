use crate::chunk::{Chunk, Instruction};
use crate::vm::VM;

mod chunk;
mod vm;

fn main() {
    let chunk = Chunk {
        constants: vec![1, 2, 3],
        code: vec![
            Instruction::CONSTANT(0),
            Instruction::CONSTANT(1),
            Instruction::SUBTRACT,
            Instruction::CONSTANT(2),
            Instruction::MULTIPLY
        ],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    println!("Result {}", result)
}
