use yggdrasil_rt::Stack;

#[test]
fn snapshot_with_empty() {
    let mut stack = Stack::new();

    stack.snapshot();
    // []
    assert!(stack.is_empty());
    // [0]
    stack.push(0);
    stack.restore();
    assert!(stack.is_empty());
}

#[test]
fn snapshot_twice() {
    let mut stack = Stack::new();

    stack.push(0);

    stack.snapshot();
    stack.snapshot();
    stack.restore();
    stack.restore();

    assert_eq!(stack[0..stack.len()], [0]);
}
#[test]
fn restore_without_snapshot() {
    let mut stack = Stack::new();

    stack.push(0);
    stack.restore();

    assert_eq!(stack[0..stack.len()], [0; 0]);
}

#[test]
fn snapshot_pop_restore() {
    let mut stack = Stack::new();

    stack.push(0);
    stack.snapshot();
    stack.pop();
    stack.restore();

    assert_eq!(stack[0..stack.len()], [0]);
}

#[test]
fn snapshot_pop_push_restore() {
    let mut stack = Stack::new();

    stack.push(0);
    stack.snapshot();
    stack.pop();
    stack.push(1);
    stack.restore();

    assert_eq!(stack[0..stack.len()], [0]);
}

#[test]
fn snapshot_push_pop_restore() {
    let mut stack = Stack::new();

    stack.push(0);
    stack.snapshot();
    stack.push(1);
    stack.push(2);
    stack.pop();
    stack.restore();

    assert_eq!(stack[0..stack.len()], [0]);
}

#[test]
fn snapshot_push_clear() {
    let mut stack = Stack::new();

    stack.push(0);
    stack.snapshot();
    stack.push(1);
    stack.clear_snapshot();

    assert_eq!(stack[0..stack.len()], [0, 1]);
}

#[test]
fn snapshot_pop_clear() {
    let mut stack = Stack::new();

    stack.push(0);
    stack.push(1);
    stack.snapshot();
    stack.pop();
    stack.clear_snapshot();

    assert_eq!(stack[0..stack.len()], [0]);
}

#[test]
fn stack_ops() {
    let mut stack = Stack::new();

    // []
    assert!(stack.is_empty());
    assert_eq!(stack.peek(), None);
    assert_eq!(stack.pop(), None);

    // [0]
    stack.push(0);
    assert!(!stack.is_empty());
    assert_eq!(stack.peek(), Some(&0));

    // [0, 1]
    stack.push(1);
    assert!(!stack.is_empty());
    assert_eq!(stack.peek(), Some(&1));

    // [0]
    assert_eq!(stack.pop(), Some(1));
    assert!(!stack.is_empty());
    assert_eq!(stack.peek(), Some(&0));

    // [0, 2]
    stack.push(2);
    assert!(!stack.is_empty());
    assert_eq!(stack.peek(), Some(&2));

    // [0, 2, 3]
    stack.push(3);
    assert!(!stack.is_empty());
    assert_eq!(stack.peek(), Some(&3));

    // Take a snapshot of the current stack
    // [0, 2, 3]
    stack.snapshot();

    // [0, 2]
    assert_eq!(stack.pop(), Some(3));
    assert!(!stack.is_empty());
    assert_eq!(stack.peek(), Some(&2));

    // Take a snapshot of the current stack
    // [0, 2]
    stack.snapshot();

    // [0]
    assert_eq!(stack.pop(), Some(2));
    assert!(!stack.is_empty());
    assert_eq!(stack.peek(), Some(&0));

    // []
    assert_eq!(stack.pop(), Some(0));
    assert!(stack.is_empty());

    // Test backtracking
    // [0, 2]
    stack.restore();
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(0));
    assert_eq!(stack.pop(), None);

    // Test backtracking
    // [0, 2, 3]
    stack.restore();
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(0));
    assert_eq!(stack.pop(), None);
}
