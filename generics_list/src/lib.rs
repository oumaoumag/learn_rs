#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Box<Node<T>>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    // creats an empty list
    pub fn new() -> List<T> {
        List { head: None }
    }

    // Adds an element to the beginning of the list
    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(), 
        });
        self.head = Some(new_node);
    }

    // removes te first element from the list
    pub fn pop(&mut self) {
        if let Some(mut boxed_node) = self.head.take() {
            self.head = boxed_node.next.take();
        }
    }

    // Counts the number of elements in the list
    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;

        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }
        count
    }
}