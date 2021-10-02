pub enum List<T> {
    Filled(Box<Node<T>>),
    Empty,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List::Empty
    }

    pub fn push(&mut self, data: T) {
        let mut new_node = Node {
            data: data,
            next: None,
        };

        if let List::Filled(node) = *self {
            new_node.next = Some(node);
        };

        self = &mut List::Filled(new_node);
    }
}
