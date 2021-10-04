pub enum List<'a, T> where T: Copy {
    Filled(Box<Node<T> + 'a>),
    Empty,
}

pub struct Node<'a, T> where T: Copy {
    pub data: T,
    pub next: Option<Box<Node<T> + 'a>>,
}

impl<'a, T> Iterator for List<'a, T> where T: Copy {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            List::Empty => None,
            List::Filled(ref node) => Some(node.data),
        }
    }
}

impl<'a, T> List<'a, T> where T: Copy {
    pub fn new() -> Self {
        List::Empty
    }

    pub fn push(mut self, data: T) {
        let new_node = Node {
            data: data,
            next: match self {
                List::Filled(boxed_node) => Some(&'a node),
                List::Empty => None,
            },
        };

        self = List::Filled(Box::new(new_node));
    }

    pub fn pop(mut self) -> Option<T> {
        match self {
            List::Empty => None,
            List::Filled(boxed_node) => {
                let data = boxed_node.data;
                self = match boxed_node.next {
                    None => List::Empty,
                    Some(next_node) => List::Filled(next_node),
                };
                Some(data)
            },
        }
    }
}
