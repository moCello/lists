use std::mem;

pub struct List<T>
where
    T: Copy,
{
    head: Option<Box<Node<T>>>,
}

struct Node<T>
where
    T: Copy,
{
    data: T,
    next: Option<Box<Node<T>>>,
}

// impl<T> IntoIterator for List<T> where T: Copy {
//     type Item = T;
//     type Iterator = Node<T>

//     fn into_iter(self) -> Self::IntoIter {
//         match self.head {
//             None => None,
//             Some(node) => Some(node.data),
//         }
//     }
// }

impl<T> Drop for List<T>
where
    T: Copy,
{
    fn drop(&mut self) {
        let mut walk = mem::replace(&mut self.head, None);
        while let Some(boxed_node) = walk {
            walk = boxed_node.next;
        }
    }
}

impl<T> List<T>
where
    T: Copy,
{
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, data: T) {
        let new_node = Node {
            data,
            next: mem::replace(&mut self.head, None),
        };
        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(boxed_node) => {
                let data = boxed_node.data;
                self.head = boxed_node.next;
                Some(data)
            }
        }
    }
}
