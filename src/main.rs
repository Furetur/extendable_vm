use crate::chunk::{Chunk, ChunkConstant, Instruction};
use crate::vm::VM;
use crate::chunk_parser::ChunkParser;

mod chunk;
mod jexobject;
mod jexvalues;
mod operators;
mod string_interner;
mod vm;
mod chunk_parser;

fn main() {
    let path = std::env::args().nth(1).expect("Filepath not given");
    let chunk = ChunkParser::parse_file(&path);
    match chunk {
        Err(e) => panic!("Could not parse file: {}", e),
        Ok(chunk) => {
            let mut vm = VM::new();
            vm.run(&chunk);
        }
    }
}
