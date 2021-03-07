use jex_vm::chunk::{Chunk, Instruction, ChunkConstant};
use jex_vm::vm::VM;

#[test]
fn it_should_get_local_variables_from_stack() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::INT(100), ChunkConstant::INT(-1)],
        code: vec![
            // A local variable with value 0 is at slot: 0
            Instruction::Constant(0),
            Instruction::Constant(1),
            Instruction::GetLocal(0),
        ],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert_eq!(100, result.unwrap().as_int())
}

#[test]
fn it_should_set_local_variables_new_values_from_stack() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::INT(100), ChunkConstant::from_str("new value")],
        code: vec![
            // A local variable with value 0 is at slot: 0
            Instruction::Constant(0),
            Instruction::Constant(1),
            Instruction::SetLocal(0), // eats the previous value on stack leaving only the variable
        ],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert_eq!("new value", result.unwrap().as_str())
}

#[test]
fn it_should_set_and_get_local_variables_new_values_from_stack() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::INT(100), ChunkConstant::from_str("new value"), ChunkConstant::INT(-1)],
        code: vec![
            // A local variable with value 0 is at slot: 0
            Instruction::Constant(0),
            Instruction::Constant(2), // pollutes the stack with -1
            Instruction::Constant(1),
            Instruction::SetLocal(0), // eats the previous value on stack leaving -1 at top
            Instruction::GetLocal(0) // should get "new value" from the variable
        ],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert_eq!("new value", result.unwrap().as_str())
}

// Temporary

#[test]
#[should_panic]
fn it_should_panic_if_trying_to_get_non_existing_local() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::INT(100)],
        code: vec![
            Instruction::Constant(0),
            Instruction::Constant(0),
            Instruction::Constant(1),
            Instruction::GetLocal(100)
        ],
    };
    let mut vm = VM::new();
    vm.run(&chunk);
}

#[test]
#[should_panic]
fn it_should_panic_if_trying_to_set_non_existing_local() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::INT(100)],
        code: vec![
            Instruction::Constant(0),
            Instruction::Constant(0),
            Instruction::Constant(1),
            Instruction::SetLocal(100)
        ],
    };
    let mut vm = VM::new();
    vm.run(&chunk);
}