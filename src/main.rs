use crate::bytecode::bytecode_reader::BytecodeReader;
use crate::bytecode::chunk_parser::ChunkParser;
use crate::vm::VM;

mod bytecode;
mod operators;
mod string_interner;
mod values;
mod vm;

fn main() {
    let path = std::env::args().nth(1).expect("Filepath not given");
    let mut reader = BytecodeReader::from_file(&path).expect("File not found");
    let chunk = ChunkParser::new(&mut reader).parse();
    let mut vm = VM::new();
    vm.run(&chunk);
}
