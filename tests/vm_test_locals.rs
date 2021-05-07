use jex_vm::jex::bytecode_constants::JexConstant;
use jex_vm::jex::instructions::op_codes::JexOpCode;
use run::code::{TestChunk, TestInstruction};
use run::run_jex::{run_chunk, run_instructions};

mod run;

#[test]
fn it_should_get_local_variables_from_stack() {
    let result = run_chunk(TestChunk {
        constants: vec![JexConstant::Int(1), JexConstant::Int(2)],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![0],
            },
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![1],
            },
            TestInstruction {
                op_code: JexOpCode::GetLocal,
                args: vec![1], // because 0 is <script>
            }
        ],
    });

    assert_eq!(1, result.unwrap().as_int().unwrap())
}

#[test]
fn it_should_set_local_variables_new_values_from_stack() {
    let result = run_chunk(TestChunk {
        constants: vec![JexConstant::Int(1), JexConstant::Int(2), JexConstant::from_str("abc")],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![0],
            },
            TestInstruction { // bury the L1
                op_code: JexOpCode::Constant,
                args: vec![1],
            },
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![2],
            },
            TestInstruction { // sets L1 to "abc"
                op_code: JexOpCode::SetLocal,
                args: vec![1], // because 0 is <script>
            },
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![1],
            },
            TestInstruction {
                op_code: JexOpCode::GetLocal,
                args: vec![1],
            },
        ],
    });

    assert_eq!("abc", result.unwrap().as_string().unwrap())
}


// Temporary

#[test]
#[should_panic]
fn it_should_panic_if_trying_to_get_non_existing_local() {
    run_chunk(TestChunk {
        constants: vec![JexConstant::Int(1)],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![0],
            },
            TestInstruction {
                op_code: JexOpCode::GetLocal,
                args: vec![100],
            },
        ],
    });
}

#[test]
#[should_panic]
fn it_should_panic_if_trying_to_set_non_existing_local() {
    run_chunk(TestChunk {
        constants: vec![JexConstant::Int(1)],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![0],
            },
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![0],
            },
            TestInstruction {
                op_code: JexOpCode::SetLocal,
                args: vec![100],
            },
        ],
    });
}
