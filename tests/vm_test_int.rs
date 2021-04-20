use jex_vm::bytecode::chunk::{Chunk, ChunkConstant};
use jex_vm::bytecode::instructions::Instruction;
use jex_vm::runtime::vm::VM;

#[test]
fn it_adds_two_numbers() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::INT(1), ChunkConstant::INT(2)],
        code: vec![
            Instruction::Constant(0),
            Instruction::Constant(1),
            Instruction::Add,
        ],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert_eq!(3, result.unwrap().as_int())
}

#[test]
fn it_should_subtract_two_numbers() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::INT(10), ChunkConstant::INT(2)],
        code: vec![
            Instruction::Constant(0),
            Instruction::Constant(1),
            Instruction::Subtract,
        ],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert_eq!(8, result.unwrap().as_int())
}

#[test]
fn it_should_multiply_two_numbers() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::INT(10), ChunkConstant::INT(2)],
        code: vec![
            Instruction::Constant(0),
            Instruction::Constant(1),
            Instruction::Multiply,
        ],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert_eq!(20, result.unwrap().as_int())
}

#[test]
fn it_should_divide_two_numbers() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::INT(11), ChunkConstant::INT(2)],
        code: vec![
            Instruction::Constant(0),
            Instruction::Constant(1),
            Instruction::Divide,
        ],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert_eq!(5, result.unwrap().as_int())
}

#[test]
fn it_strictly_compares_two_numbers() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::INT(11), ChunkConstant::INT(2)],
        code: vec![
            Instruction::Constant(0),
            Instruction::Constant(1),
            Instruction::Greater,
        ],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert!(result.unwrap().as_bool())
}

#[test]
fn it_should_say_that_3_is_not_greater_than_3() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::INT(3), ChunkConstant::INT(3)],
        code: vec![
            Instruction::Constant(0),
            Instruction::Constant(1),
            Instruction::Greater,
        ],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert!(!result.unwrap().as_bool())
}

#[test]
fn it_should_say_that_3_is_not_less_than_3() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::INT(3), ChunkConstant::INT(3)],
        code: vec![
            Instruction::Constant(0),
            Instruction::Constant(1),
            Instruction::Less,
        ],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert!(!result.unwrap().as_bool())
}

#[test]
fn it_should_say_that_negative10_is_less_than_100() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::INT(-10), ChunkConstant::INT(100)],
        code: vec![
            Instruction::Constant(0),
            Instruction::Constant(1),
            Instruction::Less,
        ],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert!(result.unwrap().as_bool())
}

#[test]
fn it_should_consider_3_equal_to_3() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::INT(3), ChunkConstant::INT(3)],
        code: vec![
            Instruction::Constant(0),
            Instruction::Constant(1),
            Instruction::Equal,
        ],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert!(result.unwrap().as_bool())
}

#[test]
fn it_should_consider_3_not_equal_to_4() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::INT(3), ChunkConstant::INT(4)],
        code: vec![
            Instruction::Constant(0),
            Instruction::Constant(1),
            Instruction::Equal,
        ],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert!(!result.unwrap().as_bool())
}

#[test]
fn it_should_negate_10() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::INT(10)],
        code: vec![Instruction::Constant(0), Instruction::Negate],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert_eq!(-10, result.unwrap().as_int())
}

// Temporary tests

#[test]
#[should_panic]
fn it_should_panic_if_ints_are_logically_negated() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::INT(8)],
        code: vec![Instruction::Constant(0), Instruction::Not],
    };
    let mut vm = VM::new();
    vm.run(&chunk);
}
