use jex_vm::machine::stack::Stack;

#[test]
fn initially_peak_should_return_none() {
    let mut stack: Stack<i32> = Stack::empty();
    assert!(stack.peek().is_none())
}

#[test]
fn pushed_value_should_be_peekable() {
    let mut stack: Stack<i32> = Stack::empty();
    stack.push(10);
    assert_eq!(10, *stack.peek().unwrap())
}

#[test]
fn pop_should_return_pushed_value() {
    let mut stack: Stack<i32> = Stack::empty();
    stack.push(100);
    stack.push_call_frame(0, 0); // push call frame to fool security
    stack.push(10);
    assert_eq!(10, stack.pop().unwrap())
}

#[test]
fn after_push_push_push_stack_peek_from_top_2_should_return_first_value() {
    let mut stack: Stack<i32> = Stack::empty();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    assert_eq!(1, *stack.peek_from_top(2).unwrap())
}

#[test]
fn get_local_0_should_return_the_first_element_in_call_frame() {
    let mut stack: Stack<i32> = Stack::empty();
    stack.push(1);
    stack.push_call_frame(0, 0);
    stack.push(2);
    assert_eq!(1, *stack.get_local(0).unwrap())
}

#[test]
fn get_local_0_should_return_the_first_element_in_call_frame_even_when_there_are_many_elements_before_frame() {
    let mut stack: Stack<i32> = Stack::empty();
    for _ in 0..100 {
        stack.push(-1);
    }
    stack.push(1);
    stack.push_call_frame(0, 0);
    stack.push(2);
    assert_eq!(1, *stack.get_local(0).unwrap())
}

#[test]
fn call_frames_mimic_function_calls() {
    let mut stack: Stack<i32> = Stack::empty();
    stack.push(0); // sum function
    stack.push(1); // arg1
    stack.push(2); // arg2
    stack.push_call_frame(0, 2);
    assert_eq!(0, *stack.get_local(0).unwrap());
    assert_eq!(1, *stack.get_local(1).unwrap());
    assert_eq!(2, *stack.get_local(2).unwrap());
}

#[test]
fn call_frames_mimic_nested_function_calls() {
    let mut stack: Stack<i32> = Stack::empty();
    stack.push(0); // sum function
    stack.push(1); // arg1
    stack.push(2); // arg2
    stack.push_call_frame(0, 2);
    stack.push(10); // another function
    stack.push(11); // arg0
    stack.push_call_frame(1, 1);
    assert_eq!(10, *stack.get_local(0).unwrap());
    assert_eq!(11, *stack.get_local(1).unwrap());
}

#[test]
fn discard_call_frame_should_remove_all_local_values() {
    let mut stack: Stack<i32> = Stack::empty();
    stack.push(0); // function
    stack.push_call_frame(0, 0);
    stack.push(1); // nested function call
    stack.push(2); // arg
    stack.push_call_frame(1, 1);
    for _ in 0..100 {
        stack.push(10); // data in nested function call
    }
    stack.discard_call_frame();
    assert_eq!(0, *stack.peek().unwrap());
}
