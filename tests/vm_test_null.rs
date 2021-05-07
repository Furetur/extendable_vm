use jex_vm::jex::bytecode_constants::JexConstant;
use jex_vm::jex::instructions::op_codes::JexOpCode;
use run::code::{TestChunk, TestInstruction};
use run::run_jex::{run_chunk, run_instructions};

mod run;

#[test]
fn null_should_not_be_equal_to_int() {
    let result = run_chunk(TestChunk {
        constants: vec![JexConstant::Int(0)],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![0],
            },
            TestInstruction::new(JexOpCode::Null),
            TestInstruction::new(JexOpCode::Equal),
        ],
    });

    assert!(!result.unwrap().as_bool().unwrap());
}

#[test]
fn null_should_not_be_equal_to_bool() {
    let result = run_instructions(vec![
        TestInstruction::new(JexOpCode::Null),
        TestInstruction::new(JexOpCode::True),
        TestInstruction::new(JexOpCode::Equal),
    ]);

    assert!(!result.unwrap().as_bool().unwrap());
}

#[test]
fn null_should_be_equal_to_null() {
    let result = run_instructions(vec![
        TestInstruction::new(JexOpCode::Null),
        TestInstruction::new(JexOpCode::Null),
        TestInstruction::new(JexOpCode::Equal),
    ]);

    assert!(result.unwrap().as_bool().unwrap())
}

// TODO: should add tests that should panic when anything is done with null
