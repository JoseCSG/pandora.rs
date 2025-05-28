use crate::utils::node::Node;
pub struct Stack<T> {
    top: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { top: None, size: 0 }
    }

    pub fn push(&mut self, value: T) {
        let mut new_node = Box::new(Node::new(value));
        new_node.set_next(self.top.take());
        self.top = Some(new_node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(mut top_node) = self.top.take() {
            self.top = top_node.get_next();
            self.size -= 1;
            Some(top_node.value)
        } else {
            None
        }
    }

    pub fn top_mut(&mut self) -> Option<&mut T> {
        self.top.as_mut().map(|node| &mut node.value)
    }

    pub fn top(&self) -> Option<&T> {
        self.top.as_ref().map(|node| &node.value)
    }

    pub fn is_empty(&self) -> bool {
        self.top.is_none()
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

// Implement Drop to prevent stack overflow on recursive destruction
impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut current = self.top.take();
        while let Some(mut node) = current {
            current = node.get_next();
        }
    }
}
