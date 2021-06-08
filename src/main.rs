use extendable_vm::jex::constant_parsers::JEX_CONSTANT_PARSERS;
use extendable_vm::jex::instructions::types::JexInstructionTable;
use extendable_vm::jex::instructions::JEX_INSTRUCTIONS;
use extendable_vm::jex::types::JexMachine;
use extendable_vm::machine::instruction_table::InstructionTable;
use extendable_vm::machine::machine::Machine;
use extendable_vm::machine::parsing::code_parser::CodeParser;
use extendable_vm::machine::parsing::constant_parser::ConstantParserTable;
use extendable_vm::machine::parsing::raw_bytes::RawBytes;

fn main() {
    let path = std::env::args().nth(1).expect("Filepath not given");
    // read file
    let bytes = RawBytes::from_file(&path).expect("File cannot be opened");
    // build parser
    let const_parser_table = ConstantParserTable::with_parsers(&JEX_CONSTANT_PARSERS);
    let parser = CodeParser::new(&const_parser_table);
    // parse file
    let code = parser.parse(&bytes).unwrap_or_else(|e| panic!("{}", e));
    println!("{:?}", code);
    // build machine
    let instruction_table: JexInstructionTable = InstructionTable::with_instructions(
        JEX_INSTRUCTIONS
            .to_vec()
            .iter()
            .map(|i| i.clone())
            .collect(),
    );
    // run machine
    let mut machine: JexMachine = Machine::new(&code, &instruction_table);
    machine.push_frame(0, "<script>".to_string(), 0);
    let finished_gracefully = machine.start();
    if !finished_gracefully {
        println!("There was an exception!");
    }
}
