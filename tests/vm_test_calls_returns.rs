use extendable_vm::jex::bytecode_constants::JexConstant;
use extendable_vm::jex::instructions::op_codes::JexOpCode;

use run::code::{TestChunk, TestInstruction};
use run::run_jex::{run_chunk, run_chunks, run_instructions};

mod run;

#[test]
fn should_jump_to_another_chunk() {
    let result = run_chunks(vec![
        TestChunk {
            constants: vec![JexConstant::Function { chunk_id: 1 }],
            instructions: vec![
                TestInstruction {
                    op_code: JexOpCode::Constant,
                    args: vec![0],
                },
                TestInstruction {
                    op_code: JexOpCode::Call,
                    args: vec![0],
                },
            ],
        },
        TestChunk {
            constants: vec![JexConstant::from_str("name"), JexConstant::Int(0)],
            instructions: vec![TestInstruction::new(JexOpCode::False)],
        },
    ]);
    assert_eq!(false, result.unwrap().as_bool().unwrap())
}

#[test]
fn sum_function_should_work() {
    let result = run_chunks(vec![
        TestChunk {
            constants: vec![
                JexConstant::Function { chunk_id: 1 },
                JexConstant::Int(10),
                JexConstant::Int(12),
            ],
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
                    op_code: JexOpCode::Constant,
                    args: vec![2],
                },
                TestInstruction {
                    op_code: JexOpCode::Call,
                    args: vec![2],
                },
            ],
        },
        TestChunk {
            constants: vec![JexConstant::from_str("sum"), JexConstant::Int(2)],
            instructions: vec![
                TestInstruction::new(JexOpCode::Add),
                TestInstruction::new(JexOpCode::Return),
            ],
        },
    ]);
    assert_eq!(22, result.unwrap().as_int().unwrap())
}

#[test]
fn nested_function_calls_should_work() {
    let result = run_chunks(vec![
        TestChunk {
            constants: vec![JexConstant::Function { chunk_id: 2 }, JexConstant::Int(10)],
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
                    op_code: JexOpCode::Call,
                    args: vec![1],
                },
            ],
        },
        TestChunk {
            constants: vec![
                JexConstant::from_str("twice"),
                JexConstant::Int(1),
                JexConstant::Int(2),
            ],
            instructions: vec![
                TestInstruction {
                    op_code: JexOpCode::Constant,
                    args: vec![2],
                },
                TestInstruction::new(JexOpCode::Multiply),
                TestInstruction::new(JexOpCode::Return),
            ],
        },
        TestChunk {
            constants: vec![
                JexConstant::from_str("inc_twiced"),
                JexConstant::Int(1),
                JexConstant::Function { chunk_id: 1 },
            ],
            instructions: vec![
                TestInstruction {
                    op_code: JexOpCode::Constant,
                    args: vec![2],
                },
                TestInstruction {
                    op_code: JexOpCode::GetLocal,
                    args: vec![1],
                },
                TestInstruction {
                    op_code: JexOpCode::Call,
                    args: vec![1],
                },
                TestInstruction {
                    op_code: JexOpCode::Constant,
                    args: vec![1],
                },
                TestInstruction::new(JexOpCode::Add),
                TestInstruction::new(JexOpCode::Return),
            ],
        },
    ]);
    assert_eq!(21, result.unwrap().as_int().unwrap())
}

#[should_panic]
#[test]
fn should_panic_if_call_is_performed_with_0_args_but_chunk_requires_1() {
    run_chunks(vec![
        TestChunk {
            constants: vec![JexConstant::Function { chunk_id: 1 }],
            instructions: vec![
                TestInstruction {
                    op_code: JexOpCode::Constant,
                    args: vec![0],
                },
                TestInstruction {
                    op_code: JexOpCode::Call,
                    args: vec![0],
                },
            ],
        },
        TestChunk {
            constants: vec![JexConstant::from_str("name"), JexConstant::Int(1)],
            instructions: vec![TestInstruction::new(JexOpCode::False)],
        },
    ]);
}

#[should_panic]
#[test]
fn should_panic_if_callee_chunk_has_no_name() {
    run_chunks(vec![
        TestChunk {
            constants: vec![JexConstant::Function { chunk_id: 1 }],
            instructions: vec![
                TestInstruction {
                    op_code: JexOpCode::Constant,
                    args: vec![0],
                },
                TestInstruction {
                    op_code: JexOpCode::Call,
                    args: vec![0],
                },
            ],
        },
        TestChunk {
            constants: vec![JexConstant::Int(2), JexConstant::Int(0)],
            instructions: vec![TestInstruction::new(JexOpCode::False)],
        },
    ]);
}
