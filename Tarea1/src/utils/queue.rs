use std::collections::VecDeque;

pub struct Queue<T> {
    queue: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            queue: VecDeque::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.queue.is_empty();
    }

    pub fn length(&self) -> usize {
        self.queue.len()
    }

    pub fn push(&mut self, item: T) {
        self.queue.push_back(item);
    }
    pub fn pop(&mut self) {
        self.queue.pop_front();
    }
    pub fn peek(&self) -> Option<&T> {
        self.queue.front()
    }
}
