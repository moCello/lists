use lists::persistent_stack::List as Stack;

#[test]
fn basic () {
    let stack_1 = Stack::new();
    let stack_2 = stack_1.prepend(1).prepend(2).prepend(3);
    assert_eq!(stack_1.head(), None);
    assert_eq!(stack_2.head(), Some(&3));

    let stack_1 = stack_2.tail();
    assert_eq!(stack_1.head(), Some(&2));

    let stack_1 = stack_1.prepend(4);
    assert_eq!(stack_1.head(), Some(&4));
    assert_eq!(stack_2.head(), Some(&3));

    let stack_1 = stack_1.tail();
    let stack_2 = stack_2.tail();
    assert_eq!(stack_1.head(), Some(&2));
    assert_eq!(stack_2.head(), Some(&2));

    let stack_1 = stack_1.tail();
    let stack_2 = stack_2.tail();
    assert_eq!(stack_1.head(), Some(&1));
    assert_eq!(stack_2.head(), Some(&1));

    let stack_1 = stack_1.tail();
    let stack_2 = stack_2.tail();
    assert_eq!(stack_1.head(), None);
    assert_eq!(stack_2.head(), None);
}

#[test]
fn iter() {
    let stack = Stack::new().prepend(1).prepend(2).prepend(3);

    let mut iter = stack.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);
}
