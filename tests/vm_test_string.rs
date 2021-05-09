use extendable_vm::jex::bytecode_constants::JexConstant;
use extendable_vm::jex::instructions::op_codes::JexOpCode;
use run::code::{TestChunk, TestInstruction};
use run::run_jex::{run_chunk, run_instructions};

mod run;

#[test]
fn it_concats_two_strings() {
    let result = run_chunk(TestChunk {
        constants: vec![JexConstant::from_str("a"), JexConstant::from_str("b")],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![0],
            },
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![1],
            },
            TestInstruction::new(JexOpCode::Add),
        ],
    });

    assert_eq!("ab", result.unwrap().as_string().unwrap().clone())
}

#[test]
fn lexically_equal_strings_should_be_equal() {
    let result = run_chunk(TestChunk {
        constants: vec![JexConstant::from_str("abc"), JexConstant::from_str("abc")],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![0],
            },
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![1],
            },
            TestInstruction::new(JexOpCode::Equal),
        ],
    });

    assert!(result.unwrap().as_bool().unwrap());
}

#[test]
fn lexically_different_strings_should_not_be_equal() {
    let result = run_chunk(TestChunk {
        constants: vec![JexConstant::from_str("abc"), JexConstant::from_str("abcd")],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![0],
            },
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![1],
            },
            TestInstruction::new(JexOpCode::Equal),
        ],
    });

    assert!(!result.unwrap().as_bool().unwrap());
}

// TODO: should add panic tests
