use jex_vm::jex::bytecode_constants::JexConstant;
use jex_vm::jex::instructions::op_codes::JexOpCode;
use run::code::{TestChunk, TestInstruction};
use run::run_jex::{run_chunk, run_instructions};

mod run;

#[test]
fn it_declare_and_get_global_variable_from_const() {
    let result = run_chunk(TestChunk {
        constants: vec![
            JexConstant::Int(100),
            JexConstant::from_str("varname"),
            JexConstant::Int(0),
        ],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![0],
            },
            TestInstruction {
                op_code: JexOpCode::DefineGlobal,
                args: vec![1],
            },
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![2],
            },
            TestInstruction {
                op_code: JexOpCode::GetGlobal,
                args: vec![1],
            },
        ],
    });
    assert_eq!(100, result.unwrap().as_int().unwrap());
}

#[test]
fn it_declare_and_get_calculated_global_variable() {
    let result = run_chunk(TestChunk {
        constants: vec![
            JexConstant::Int(100),
            JexConstant::from_str("varname"),
            JexConstant::Int(0),
        ],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![0],
            },
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![0],
            },
            TestInstruction::new(JexOpCode::Add),
            TestInstruction {
                op_code: JexOpCode::DefineGlobal,
                args: vec![1],
            },
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![2],
            },
            TestInstruction {
                op_code: JexOpCode::GetGlobal,
                args: vec![1],
            },
        ],
    });
    assert_eq!(200, result.unwrap().as_int().unwrap());
}

#[test]
fn it_declare_and_get_bool_global_variable() {
    let result = run_chunk(TestChunk {
        constants: vec![JexConstant::from_str("varname"), JexConstant::Int(0)],
        instructions: vec![
            TestInstruction::new(JexOpCode::True),
            TestInstruction {
                op_code: JexOpCode::DefineGlobal,
                args: vec![0],
            },
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![1],
            },
            TestInstruction {
                op_code: JexOpCode::GetGlobal,
                args: vec![0],
            },
        ],
    });
    assert_eq!(true, result.unwrap().as_bool().unwrap());
}

#[test]
fn it_declare_and_get_string_global_variable() {
    let result = run_chunk(TestChunk {
        constants: vec![
            JexConstant::from_str("varname"),
            JexConstant::Int(0),
            JexConstant::from_str("value"),
        ],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![2],
            },
            TestInstruction {
                op_code: JexOpCode::DefineGlobal,
                args: vec![0],
            },
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![1],
            },
            TestInstruction {
                op_code: JexOpCode::GetGlobal,
                args: vec![0],
            },
        ],
    });
    assert_eq!("value", result.unwrap().as_string().unwrap().as_str());
}

// Temporary

#[test]
#[should_panic]
fn it_should_panic_if_global_variable_name_is_not_string() {
    run_chunk(TestChunk {
        constants: vec![JexConstant::Int(0)],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![0],
            },
            TestInstruction {
                op_code: JexOpCode::DefineGlobal,
                args: vec![0],
            },
        ],
    });
}

#[test]
#[should_panic]
fn it_should_panic_if_trying_to_get_undefined_global() {
    run_chunk(TestChunk {
        constants: vec![JexConstant::from_str("varname")],
        instructions: vec![TestInstruction {
            op_code: JexOpCode::GetGlobal,
            args: vec![0],
        }],
    });
}
