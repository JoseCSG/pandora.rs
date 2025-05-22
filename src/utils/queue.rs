use std::fmt::Display;

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.elements.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.elements.is_empty() {
            None
        } else {
            Some(self.elements.remove(0))
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.first()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.elements.get(index)
    }

    pub fn set(&mut self, index: usize, value: T) {
        self.elements[index] = value;
    }

    pub fn print_elements(&self)
    where
        T: Display,
    {
        for (i, element) in self.elements.iter().enumerate() {
            println!("{} {}", i, element);
        }
    }
}
