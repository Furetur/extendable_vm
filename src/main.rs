use clap::{AppSettings, Clap};

use extendable_vm::jex::build_jex_machine;
use extendable_vm::jex::constant_parsers::JEX_CONSTANT_PARSERS;
use extendable_vm::machine::parsing::code_parser::CodeParser;
use extendable_vm::machine::parsing::constant_parser::ConstantParserTable;
use extendable_vm::machine::parsing::raw_bytes::RawBytes;

extern crate log;
extern crate pretty_env_logger;

#[derive(Clap)]
#[clap(author = "Furetur <furetur@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct CliOptions {
    #[clap(about = "Path to file that contains bytecode")]
    input_file: String,
    #[clap(short, long, about = "Print parsed bytecode chunks and constants")]
    print_parsed: bool,
}

fn main() {
    pretty_env_logger::init();

    let options: CliOptions = CliOptions::parse();
    // read file
    let bytes = RawBytes::from_file(&options.input_file).expect("File cannot be opened");
    // build parser
    let const_parser_table = ConstantParserTable::with_parsers(&JEX_CONSTANT_PARSERS);
    let parser = CodeParser::new(&const_parser_table);
    // parse file
    let code = parser.parse(&bytes).unwrap_or_else(|e| panic!("{}", e));
    if options.print_parsed {
        println!("{:?}", code);
    }
    // build machine
    let mut machine = build_jex_machine(&code);
    // start
    let finished_gracefully = machine.start();
    if !finished_gracefully {
        println!("There was an exception!");
    }
}
