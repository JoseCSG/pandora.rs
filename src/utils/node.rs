pub type NextNode<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    pub value: T,
    next: NextNode<T>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node { value, next: None }
    }
    pub fn get_next(&mut self) -> Option<Box<Node<T>>> {
        self.next.take()
    }

    pub fn set_next(&mut self, next: Option<Box<Node<T>>>) {
        self.next = next;
    }
}
