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

#[test]
fn peek() {
    let mut stack = Stack::new();

    stack.push(1);
    assert_eq!(stack.peek(), Some(&1));
    stack.pop();
    assert_eq!(stack.peek(), None);
}

#[test]
fn peek_mut() {
    let mut stack = Stack::new();

    stack.push(2);
    assert_eq!(stack.peek(), Some(&2));
    stack.peek_mut().map(|data| *data = 3);
    assert_eq!(stack.peek(), Some(&3));
}

#[test]
fn into_iter() {
    let mut stack = Stack::new();

    stack.push(42);
    stack.push(44);
    stack.push(3);
    let mut iter = stack.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(44));
    assert_eq!(iter.next(), Some(42));
    assert_eq!(iter.next(), None);
}

#[test]
fn iter() {
    let mut stack = Stack::new();

    stack.push(2);
    stack.push(4);
    stack.push(8);
    stack.push(16);
    let mut iter = stack.iter();
    assert_eq!(iter.next(), Some(&16));
    assert_eq!(iter.next(), Some(&8));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);
    assert_eq!(stack.pop(), Some(16));
}

#[test]
fn iter_mut() {
    let mut stack = Stack::new();
    stack.push(10);

    let mut iter_mut = stack.iter_mut();

    if let Some(x) = iter_mut.next() {
        assert_eq!(*x, 10);
        *x = 100;
    }

    let mut iter = stack.iter();
    assert_eq!(iter.next(), Some(&100));
}
