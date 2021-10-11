pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

// impl<T> IntoIterator for List<T> {
//     type Item = T;
//     type Iterator = Node<T>

//     fn into_iter(self) -> Self::IntoIter {
//         match self.head {
//             None => None,
//             Some(node) => Some(node.data),
//         }
//     }
// }

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut walk = self.head.take();
        while let Some(boxed_node) = walk {
            walk = boxed_node.next;
        }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, data: T) {
        let new_node = Node {
            data,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|boxed_node| {
            let data = boxed_node.data;
            self.head = boxed_node.next;
            data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|boxed_node| &boxed_node.data)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|boxed_node| &mut boxed_node.data)
    }
}
