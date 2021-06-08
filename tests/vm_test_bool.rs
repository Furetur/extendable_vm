use extendable_vm::jex::instructions::op_codes::JexOpCode;
use run::code::TestInstruction;
use run::run_jex::run_instructions;

mod run;

#[test]
fn true_and_true_should_be_equal() {
    let result = run_instructions(vec![
        TestInstruction::new(JexOpCode::True),
        TestInstruction::new(JexOpCode::True),
        TestInstruction::new(JexOpCode::Equal),
    ]);

    assert!(result.unwrap().as_bool().unwrap());
}

#[test]
fn false_and_false_should_be_equal() {
    let result = run_instructions(vec![
        TestInstruction::new(JexOpCode::False),
        TestInstruction::new(JexOpCode::False),
        TestInstruction::new(JexOpCode::Equal),
    ]);

    assert!(result.unwrap().as_bool().unwrap());
}

#[test]
fn true_and_false_should_be_equal() {
    let result = run_instructions(vec![
        TestInstruction::new(JexOpCode::False),
        TestInstruction::new(JexOpCode::True),
        TestInstruction::new(JexOpCode::Equal),
    ]);

    assert!(!result.unwrap().as_bool().unwrap());
}

#[test]
fn not_true_should_be_false() {
    let result = run_instructions(vec![
        TestInstruction::new(JexOpCode::True),
        TestInstruction::new(JexOpCode::Not),
    ]);

    assert!(!result.unwrap().as_bool().unwrap());
}

#[test]
fn not_false_should_be_true() {
    let result = run_instructions(vec![
        TestInstruction::new(JexOpCode::False),
        TestInstruction::new(JexOpCode::Not),
    ]);

    assert!(result.unwrap().as_bool().unwrap());
}

#[test]
#[should_panic]
fn it_should_panic_if_bools_are_added() {
    run_instructions(vec![
        TestInstruction::new(JexOpCode::False),
        TestInstruction::new(JexOpCode::True),
        TestInstruction::new(JexOpCode::Add),
    ]);
}

#[test]
#[should_panic]
fn it_should_panic_if_bools_are_subtracted() {
    run_instructions(vec![
        TestInstruction::new(JexOpCode::False),
        TestInstruction::new(JexOpCode::True),
        TestInstruction::new(JexOpCode::Subtract),
    ]);
}

#[test]
#[should_panic]
fn it_should_panic_if_bools_are_multiplied() {
    run_instructions(vec![
        TestInstruction::new(JexOpCode::False),
        TestInstruction::new(JexOpCode::True),
        TestInstruction::new(JexOpCode::Multiply),
    ]);
}

#[test]
#[should_panic]
fn it_should_panic_if_bools_are_divided() {
    run_instructions(vec![
        TestInstruction::new(JexOpCode::False),
        TestInstruction::new(JexOpCode::True),
        TestInstruction::new(JexOpCode::Divide),
    ]);
}

#[test]
#[should_panic]
fn it_should_panic_if_bools_are_compared_with_greater() {
    run_instructions(vec![
        TestInstruction::new(JexOpCode::False),
        TestInstruction::new(JexOpCode::True),
        TestInstruction::new(JexOpCode::Greater),
    ]);
}

#[test]
#[should_panic]
fn it_should_panic_if_bools_are_compared_with_less() {
    run_instructions(vec![
        TestInstruction::new(JexOpCode::False),
        TestInstruction::new(JexOpCode::True),
        TestInstruction::new(JexOpCode::Less),
    ]);
}

#[test]
#[should_panic]
fn it_should_panic_if_bool_is_negated() {
    run_instructions(vec![
        TestInstruction::new(JexOpCode::False),
        TestInstruction::new(JexOpCode::Negate),
    ]);
}
