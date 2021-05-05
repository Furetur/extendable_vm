use jex_vm::code::chunk::{Chunk, ChunkConstant};
use jex_vm::jex::instructions::Instruction;
use jex_vm::runtime::vm::VM;

#[test]
fn it_declare_and_get_global_variable_from_const() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::INT(100), ChunkConstant::from_str("varname")],
        code: vec![
            Instruction::Constant(0),
            Instruction::DefineGlobal(1),
            Instruction::Constant(1),
            Instruction::GetGlobal(1),
        ],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert_eq!(100, result.unwrap().as_int())
}

#[test]
fn it_declare_and_get_calculated_global_variable() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::INT(10), ChunkConstant::from_str("varname")],
        code: vec![
            Instruction::Constant(0),
            Instruction::Constant(0),
            Instruction::Multiply,
            Instruction::DefineGlobal(1),
            Instruction::Constant(1),
            Instruction::GetGlobal(1),
        ],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert_eq!(100, result.unwrap().as_int())
}

#[test]
fn it_declare_and_get_bool_global_variable() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::INT(10), ChunkConstant::from_str("varname")],
        code: vec![
            Instruction::True,
            Instruction::DefineGlobal(1),
            Instruction::Constant(1),
            Instruction::GetGlobal(1),
        ],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert_eq!(true, result.unwrap().as_bool())
}

#[test]
fn it_declare_and_get_string_global_variable() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::INT(10), ChunkConstant::from_str("varname")],
        code: vec![
            Instruction::Constant(1),
            Instruction::DefineGlobal(1),
            Instruction::Constant(0),
            Instruction::GetGlobal(1),
        ],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert_eq!("varname", result.unwrap().as_str())
}

// Temporary

#[test]
#[should_panic]
fn it_should_panic_if_global_variable_name_is_not_string() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::INT(10)],
        code: vec![Instruction::True, Instruction::DefineGlobal(0)],
    };
    let mut vm = VM::new();
    vm.run(&chunk);
}

#[test]
#[should_panic]
fn it_should_panic_if_trying_to_get_undefined_global() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::from_str("a")],
        code: vec![Instruction::True, Instruction::GetGlobal(0)],
    };
    let mut vm = VM::new();
    vm.run(&chunk);
}
