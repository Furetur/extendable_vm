use std::fmt::{Debug, Formatter};
use std::iter::Rev;
use std::slice::Iter;

pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn empty() -> Stack<T> {
        Stack { data: vec![] }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.data.last_mut()
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value)
    }

    pub fn set(&mut self, index: usize, value: T) -> Result<(), ()> {
        if index < self.len() {
            self.data[index] = value;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn get_from_top(&self, index_from_top: usize) -> Option<&T> {
        if index_from_top < self.len() {
            self.data.get(self.len() - 1 - index_from_top)
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn rev(&self) -> Rev<Iter<T>> {
        self.data.iter().rev()
    }
}

impl<T: Debug> Debug for Stack<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

#[cfg(test)]
mod tests {
    use crate::runtime::stack::Stack;

    #[test]
    fn stack_initially_should_have_0_size() {
        let stack: Stack<i32> = Stack::empty();
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn empty_stack_should_have_size_1_after_1_element_added() {
        let mut stack: Stack<i32> = Stack::empty();
        stack.push(1);
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn empty_stack_should_have_size_100_after_100_elements_added() {
        let mut stack: Stack<i32> = Stack::empty();
        for i in 0..100 {
            stack.push(i);
        }
        assert_eq!(stack.len(), 100);
    }

    #[test]
    fn empty_stack_should_return_none_on_pop() {
        let mut stack: Stack<i32> = Stack::empty();
        assert!(stack.pop().is_none());
    }

    #[test]
    fn stack_of_size_10_should_not_change_size_after_element_is_set() {
        let mut stack: Stack<i32> = Stack::empty();
        for i in 0..10 {
            stack.push(i);
        }
        stack.set(5, 100).unwrap();
        assert_eq!(stack.len(), 10);
    }

    #[test]
    fn stack_of_size_100_should_become_99_after_element_is_popped() {
        let mut stack: Stack<i32> = Stack::empty();
        for i in 0..100 {
            stack.push(i);
        }
        stack.pop();
        assert_eq!(stack.len(), 99);
    }
}
