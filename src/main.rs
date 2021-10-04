extern crate lists;
use lists::stack::List as Stack;

pub fn main() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    println!("item popped: {}", stack.pop().unwrap());
    // assert!(stack.pop().unwrap(), 4);
    // assert!(stack.pop().unwrap(), 3);
    // assert!(stack.pop().unwrap(), 2);
}
