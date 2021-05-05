use crate::bytecode::bytecode_reader::BytecodeReader;
use crate::bytecode::chunk_parser::ChunkParser;
use crate::runtime::vm::VM;

mod string_interner;

fn main() {
    let path = std::env::args().nth(1).expect("Filepath not given");
    let mut reader = BytecodeReader::from_file(&path).expect("File not found");
    let chunk = ChunkParser::new(&mut reader).parse();
    let mut vm = VM::new();
    vm.run(&chunk);
}
