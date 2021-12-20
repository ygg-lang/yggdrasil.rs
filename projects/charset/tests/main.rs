use character_set::CharacterSet;

#[test]
fn ready() {
    println!("it works!")
}

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

#[test]
fn test2() {
    let mut set = CharacterSet::default();
    set.insert(7..10);
    set.insert(11..13);
    set.insert(11..15);
    set.insert(14..20);
    set.insert(23..39);
    set.optimize();
    println!("{:#?}", set.count());
    println!("{:#?}", set);
}

#[test]
fn test3() {
    let mut set = CharacterSet::default();
    set.insert('a');
    set.optimize();
    assert!(set.contains('a'));
}