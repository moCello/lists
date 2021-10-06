extern crate lists;
use lists::stack::List as Stack;

#[test]
fn basic_functionality() {
    let mut stack = Stack::new();

    stack.push(1);
    stack.push(2);
    assert_eq!(stack.pop(), Some(2));
    stack.push(3);
    stack.push(4);
    assert_eq!(stack.pop(), Some(4));
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
}
