use crate::jex::types::JexMachine;
use crate::jex::bytecode_reader::BytecodeReader;
use crate::jex::bytecode_parser::BytecodeParser;
use crate::machine::instruction_table::InstructionTable;
use crate::jex::instructions::types::JexInstructionTable;
use crate::jex::instructions::JEX_INSTRUCTIONS;
use crate::machine::machine::Machine;

mod bytecode_constants;
mod bytecode_reader;
mod bytecode_parser;
mod values;
mod instructions;

pub mod types {
    use crate::machine::machine::Machine;
    use crate::jex::bytecode_constants::JexConstant;
    use crate::jex::values::JexValue;

    pub type JexMachine<'a> = Machine<'a, JexConstant, JexValue>;
}

pub mod make_jex {
    use crate::jex::types::JexMachine;
    use crate::jex::bytecode_reader::BytecodeReader;
    use crate::jex::bytecode_parser::BytecodeParser;
    use crate::jex::instructions::types::JexInstructionTable;
    use crate::machine::instruction_table::InstructionTable;
    use crate::jex::instructions::JEX_INSTRUCTIONS;
    use crate::machine::machine::Machine;

    pub fn make_jex_machine(path: &String) -> Result<JexMachine, ()> {
        let mut reader = BytecodeReader::from_file(path)?;
        let code = BytecodeParser::new().parse(&mut reader)?;
        let mut instruction_table: JexInstructionTable = InstructionTable::new();

        for instruction in JEX_INSTRUCTIONS {
            instruction_table.register_instruction(instruction);
        }

        Ok(Machine::new(&code, &instruction_table))
    }
}
