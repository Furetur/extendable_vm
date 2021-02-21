use crate::chunk::{Chunk, Instruction};
use crate::vm::VM;
use std::mem::size_of;
use crate::jexvalues::JexValue;
use std::mem::size_of_val;

mod chunk;
mod vm;
mod jexvalues;
mod stack;

fn main() {
    let chunk = Chunk {
        constants: vec![1, 2, 3],
        code: vec![
            Instruction::CONSTANT(2),
            Instruction::CONSTANT(2),
            Instruction::EQUAL,
            Instruction::NOT
        ],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    println!("Result {:?}", result);
}
