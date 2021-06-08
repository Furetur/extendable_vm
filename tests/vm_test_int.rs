use extendable_vm::jex::bytecode_constants::JexConstant;
use extendable_vm::jex::instructions::op_codes::JexOpCode;
use run::code::{TestChunk, TestInstruction};
use run::run_jex::run_chunk;

mod run;

#[test]
fn it_adds_two_numbers() {
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
            TestInstruction::new(JexOpCode::Add),
        ],
    });

    assert_eq!(3, result.unwrap().as_int().unwrap())
}

#[test]
fn it_should_subtract_two_numbers() {
    let result = run_chunk(TestChunk {
        constants: vec![JexConstant::Int(10), JexConstant::Int(2)],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![0],
            },
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![1],
            },
            TestInstruction::new(JexOpCode::Subtract),
        ],
    });

    assert_eq!(8, result.unwrap().as_int().unwrap())
}

#[test]
fn it_should_multiply_two_numbers() {
    let result = run_chunk(TestChunk {
        constants: vec![JexConstant::Int(10), JexConstant::Int(2)],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![0],
            },
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![1],
            },
            TestInstruction::new(JexOpCode::Multiply),
        ],
    });

    assert_eq!(20, result.unwrap().as_int().unwrap())
}

#[test]
fn it_should_divide_two_numbers() {
    let result = run_chunk(TestChunk {
        constants: vec![JexConstant::Int(11), JexConstant::Int(2)],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![0],
            },
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![1],
            },
            TestInstruction::new(JexOpCode::Divide),
        ],
    });

    assert_eq!(5, result.unwrap().as_int().unwrap())
}

#[test]
fn it_strictly_compares_two_numbers() {
    let result = run_chunk(TestChunk {
        constants: vec![JexConstant::Int(11), JexConstant::Int(2)],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![0],
            },
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![1],
            },
            TestInstruction::new(JexOpCode::Greater),
        ],
    });

    assert!(result.unwrap().as_bool().unwrap())
}

#[test]
fn it_should_say_that_3_is_not_greater_than_3() {
    let result = run_chunk(TestChunk {
        constants: vec![JexConstant::Int(3), JexConstant::Int(3)],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![0],
            },
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![1],
            },
            TestInstruction::new(JexOpCode::Greater),
        ],
    });

    assert!(!result.unwrap().as_bool().unwrap())
}

#[test]
fn it_should_say_that_3_is_not_less_than_3() {
    let result = run_chunk(TestChunk {
        constants: vec![JexConstant::Int(3), JexConstant::Int(3)],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![0],
            },
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![1],
            },
            TestInstruction::new(JexOpCode::Less),
        ],
    });

    assert!(!result.unwrap().as_bool().unwrap())
}

#[test]
fn it_should_say_that_negative10_is_less_than_100() {
    let result = run_chunk(TestChunk {
        constants: vec![JexConstant::Int(-10), JexConstant::Int(100)],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![0],
            },
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![1],
            },
            TestInstruction::new(JexOpCode::Less),
        ],
    });

    assert!(result.unwrap().as_bool().unwrap())
}

#[test]
fn it_should_consider_3_equal_to_3() {
    let result = run_chunk(TestChunk {
        constants: vec![JexConstant::Int(3), JexConstant::Int(3)],
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

    assert!(result.unwrap().as_bool().unwrap())
}

#[test]
fn it_should_consider_3_not_equal_to_4() {
    let result = run_chunk(TestChunk {
        constants: vec![JexConstant::Int(3), JexConstant::Int(4)],
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

    assert!(!result.unwrap().as_bool().unwrap())
}

#[test]
fn it_should_negate_10() {
    let result = run_chunk(TestChunk {
        constants: vec![JexConstant::Int(10)],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![0],
            },
            TestInstruction::new(JexOpCode::Negate),
        ],
    });

    assert_eq!(-10, result.unwrap().as_int().unwrap())
}

// Temporary tests

#[test]
#[should_panic]
fn it_should_panic_if_ints_are_logically_negated() {
    run_chunk(TestChunk {
        constants: vec![JexConstant::Int(10)],
        instructions: vec![
            TestInstruction {
                op_code: JexOpCode::Constant,
                args: vec![0],
            },
            TestInstruction::new(JexOpCode::Not),
        ],
    });
}
