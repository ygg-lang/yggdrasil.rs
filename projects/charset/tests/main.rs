use character_set::CharacterSet;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_ascii_range() {
    let mut set = CharacterSet::default();
    set.include('a'..'z');
    set.include('A'..'Z');
    set.include('0'..'9');
    set.optimize();
    assert_eq!(set.count(), 62);
    assert!(set.contains('a'));
    assert!(!set.contains(' '));
}

#[test]
fn test2() {
    let mut set = CharacterSet::default();
    set.include(7..10);
    set.include(11..13);
    set.include(11..15);
    set.include(14..20);
    set.include(23..39);
    set.optimize();
    println!("{:#?}", set.count());
    println!("{:#?}", set);
}

#[test]
fn test3() {
    let mut set = CharacterSet::default();
    set.include('a');
    set.optimize();
    assert!(set.contains('a'));
}
