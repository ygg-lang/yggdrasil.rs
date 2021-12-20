


```rust
#[test]
fn test_ascii_range() {
    let mut set = CharacterSet::default();
    set.insert('a'..'z');
    set.insert('A'..'Z');
    set.insert('0'..'9');
    set.optimize();
    assert_eq!(set.count(), 62);
    assert!(set.contains('a'));
    assert!(!set.contains(' '));
}
```