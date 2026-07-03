use super::*;

#[test]
fn test_is_valid() {
    assert_eq!(is_valid("()"), true);
    assert_eq!(is_valid("()[]{}"), true);
    assert_eq!(is_valid("(]"), false);
    assert_eq!(is_valid("([)]"), false);
    assert_eq!(is_valid("{[]}"), true);
}

#[test]
fn test_my_queue() {
    let mut queue = MyQueue::new();
    queue.push(1);
    queue.push(2);
    assert_eq!(queue.peek(), Some(1));
    assert_eq!(queue.pop(), Some(1));
    assert_eq!(queue.empty(), false);
    assert_eq!(queue.pop(), Some(2));
    assert_eq!(queue.empty(), true);
}
