use crate::jex::bytecode_reader::BytecodeReader;
use crate::jex::bytecode_parser::BytecodeParser;
use crate::jex::instructions::jex_instructions;
use crate::machine::machine::Machine;
use crate::machine::instruction_table::InstructionTable;
use crate::jex::instructions::types::JexInstructionTable;
use crate::jex::types::JexMachine;

mod jex;
mod machine;

fn main() {
    let path = std::env::args().nth(1).expect("Filepath not given");
    let mut reader = BytecodeReader::from_file(&path).expect("File not found");
    let code = BytecodeParser::new().parse(&mut reader).expect("CodeReadingError");
    let mut instruction_table: JexInstructionTable =
        InstructionTable::with_instructions(jex_instructions());

    let mut machine: JexMachine = Machine::new(&code, &instruction_table);
    machine.run();
}
