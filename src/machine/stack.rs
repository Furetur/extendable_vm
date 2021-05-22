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
            self.data.insert(index, value);
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
