use jex_vm::jex::bytecode_constants::JexConstant;
use jex_vm::jex::instructions::op_codes::JexOpCode;
use jex_vm::jex::values::JexFunction;
use run::code::{TestChunk, TestInstruction};
use run::run_jex::{run_chunk, run_instructions};

mod run;

#[test]
fn should_exit_if_jumped_too_far() {
    let result = run_chunk(TestChunk {
        constants: vec![JexConstant::Int(0)],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::JumpForward,
                args: vec![2],
            },
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![0],
            },
        ],
    });
    assert_eq!(JexFunction::Script, *result.unwrap().as_function().unwrap())
}

#[test]
fn should_skip_one_instruction_by_jumping() {
    let result = run_chunk(TestChunk {
        constants: vec![JexConstant::Int(0), JexConstant::Int(1)],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![0],
            },
            TestInstruction {
                op_code: JexOpCode::JumpForward,
                args: vec![2],
            },
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![1],
            },
        ],
    });
    assert_eq!(0, result.unwrap().as_int().unwrap())
}

#[test]
#[should_panic]
fn should_panic_if_jumped_before_code() {
    run_chunk(TestChunk {
        constants: vec![],
        instructions: vec![
            TestInstruction::new(JexOpCode::Null),
            TestInstruction {
                op_code: JexOpCode::JumpBackward,
                args: vec![10],
            },
        ],
    });
}

#[test]
fn should_skip_one_instruction_by_jumping_if_false() {
    let result = run_chunk(TestChunk {
        constants: vec![JexConstant::Int(0), JexConstant::Int(1)],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![0],
            },
            TestInstruction::new(JexOpCode::False),
            TestInstruction {
                op_code: JexOpCode::JumpForwardIfFalse,
                args: vec![2],
            },
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![1],
            },
        ],
    });
    assert_eq!(false, result.unwrap().as_bool().unwrap())
}

#[test]
fn should_not_skip_one_instruction_by_jumping_if_true() {
    let result = run_chunk(TestChunk {
        constants: vec![JexConstant::Int(0), JexConstant::Int(1)],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![0],
            },
            TestInstruction::new(JexOpCode::True),
            TestInstruction {
                op_code: JexOpCode::JumpForwardIfFalse,
                args: vec![2],
            },
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![1],
            },
        ],
    });
    assert_eq!(1, result.unwrap().as_int().unwrap())
}

#[test]
fn should_jump_backward() {
    let result = run_chunk(TestChunk {
        constants: vec![JexConstant::Int(0), JexConstant::Int(1)],
        instructions: vec![
            TestInstruction {
                // jump over false and exit
                op_code: JexOpCode::JumpForward,
                args: vec![3],
            },
            TestInstruction::new(JexOpCode::False), // false
            TestInstruction {
                // exit
                op_code: JexOpCode::JumpForward,
                args: vec![100],
            },
            TestInstruction {
                // pollutes stack. if jump backward fails then 0 will be on the top
                op_code: JexOpCode::Constant,
                args: vec![0],
            },
            TestInstruction {
                op_code: JexOpCode::JumpBackward,
                args: vec![7],
            },
        ],
    });
    assert_eq!(false, result.unwrap().as_bool().unwrap())
}
