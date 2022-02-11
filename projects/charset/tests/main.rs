use character_set::CharacterSet;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_ascii_range() {
    let mut set = CharacterSet::default();
    set.include('a'..'z').unwrap();
    set.include('A'..'Z').unwrap();
    set.include('0'..'9').unwrap();
    println!("{:#}", set);
    assert_eq!(set.count(), 62);
    assert!(set.contains('a'));
    assert!(!set.contains(' '));
}

#[test]
fn test2() {
    let mut set = CharacterSet::default();
    set.include(7..10).unwrap();
    set.include(11..13).unwrap();
    set.include(11..15).unwrap();
    set.include(14..20).unwrap();
    set.include(23..39).unwrap();
    println!("{}", set.count());
    println!("{:#?}", set);
}

#[test]
fn test3() {
    let mut set = CharacterSet::default();
    set.include('a').unwrap();
    println!("{:#}", set);
    assert!(set.contains('a'));
}
