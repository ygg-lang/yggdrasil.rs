
use super::*;

#[test]
fn empty() {
    let input = "";
    assert_eq!(Position::new(input, 0).unwrap().match_string(""), true);
    assert_eq!(!Position::new(input, 0).unwrap().match_string("a"), true);
}

#[test]
fn parts() {
    let input = "asdasdf";

    assert_eq!(Position::new(input, 0).unwrap().match_string("asd"), true);
    assert_eq!(Position::new(input, 3).unwrap().match_string("asdf"), true);
}

#[test]
fn line_col() {
    let input = "a\rb\nc\r\nd嗨";

    assert_eq!(Position::new(input, 0).unwrap().line_col(), (1, 1));
    assert_eq!(Position::new(input, 1).unwrap().line_col(), (1, 2));
    assert_eq!(Position::new(input, 2).unwrap().line_col(), (1, 3));
    assert_eq!(Position::new(input, 3).unwrap().line_col(), (1, 4));
    assert_eq!(Position::new(input, 4).unwrap().line_col(), (2, 1));
    assert_eq!(Position::new(input, 5).unwrap().line_col(), (2, 2));
    assert_eq!(Position::new(input, 6).unwrap().line_col(), (2, 3));
    assert_eq!(Position::new(input, 7).unwrap().line_col(), (3, 1));
    assert_eq!(Position::new(input, 8).unwrap().line_col(), (3, 2));
    assert_eq!(Position::new(input, 11).unwrap().line_col(), (3, 3));
}

#[test]
fn line_of() {
    let input = "a\rb\nc\r\nd嗨";

    assert_eq!(Position::new(input, 0).unwrap().line_of(), "a\rb\n");
    assert_eq!(Position::new(input, 1).unwrap().line_of(), "a\rb\n");
    assert_eq!(Position::new(input, 2).unwrap().line_of(), "a\rb\n");
    assert_eq!(Position::new(input, 3).unwrap().line_of(), "a\rb\n");
    assert_eq!(Position::new(input, 4).unwrap().line_of(), "c\r\n");
    assert_eq!(Position::new(input, 5).unwrap().line_of(), "c\r\n");
    assert_eq!(Position::new(input, 6).unwrap().line_of(), "c\r\n");
    assert_eq!(Position::new(input, 7).unwrap().line_of(), "d嗨");
    assert_eq!(Position::new(input, 8).unwrap().line_of(), "d嗨");
    assert_eq!(Position::new(input, 11).unwrap().line_of(), "d嗨");
}

#[test]
fn line_of_empty() {
    let input = "";

    assert_eq!(Position::new(input, 0).unwrap().line_of(), "");
}

#[test]
fn line_of_new_line() {
    let input = "\n";

    assert_eq!(Position::new(input, 0).unwrap().line_of(), "\n");
}

#[test]
fn line_of_between_new_line() {
    let input = "\n\n";

    assert_eq!(Position::new(input, 1).unwrap().line_of(), "\n");
}

fn measure_skip(input: &str, pos: usize, n: usize) -> Option<usize> {
    let mut p = Position::new(input, pos).unwrap();
    if p.skip(n) { Some(p.pos - pos) } else { None }
}

#[test]
fn skip_empty() {
    let input = "";

    assert_eq!(measure_skip(input, 0, 0), Some(0));
    assert_eq!(measure_skip(input, 0, 1), None);
}

#[test]
fn skip() {
    let input = "d嗨";

    assert_eq!(measure_skip(input, 0, 0), Some(0));
    assert_eq!(measure_skip(input, 0, 1), Some(1));
    assert_eq!(measure_skip(input, 1, 1), Some(3));
}

#[test]
fn skip_until() {
    let input = "ab ac";
    let pos = Position::from_start(input);

    let mut test_pos = pos.clone();
    test_pos.skip_until(&["a", "b"]);
    assert_eq!(test_pos.pos(), 0);

    test_pos = pos.clone();
    test_pos.skip_until(&["b"]);
    assert_eq!(test_pos.pos(), 1);

    test_pos = pos.clone();
    test_pos.skip_until(&["ab"]);
    assert_eq!(test_pos.pos(), 0);

    test_pos = pos.clone();
    test_pos.skip_until(&["ac", "z"]);
    assert_eq!(test_pos.pos(), 3);

    test_pos = pos.clone();
    assert!(!test_pos.skip_until(&["z"]));
    assert_eq!(test_pos.pos(), 5);
}

#[test]
fn match_range() {
    let input = "b";

    assert_eq!(Position::new(input, 0).unwrap().match_char_range('a'..'c'), true);
    assert_eq!(Position::new(input, 0).unwrap().match_char_range('b'..'b'), true);
    assert_eq!(!Position::new(input, 0).unwrap().match_char_range('a'..'a'), true);
    assert_eq!(!Position::new(input, 0).unwrap().match_char_range('c'..'c'), true);
    assert_eq!(Position::new(input, 0).unwrap().match_char_range('a'..'嗨'), true);
}

#[test]
fn match_insensitive() {
    let input = "AsdASdF";

    assert_eq!(Position::new(input, 0).unwrap().match_insensitive("asd"), true);
    assert_eq!(Position::new(input, 3).unwrap().match_insensitive("asdf"), true);
}

#[test]
fn cmp() {
    let input = "a";
    let start = Position::from_start(input);
    let mut end = start.clone();

    assert!(end.skip(1));
    let result = start.cmp(&end);

    assert_eq!(result, Ordering::Less);
}

#[test]
#[should_panic]
fn cmp_panic() {
    let input1 = "a";
    let input2 = "b";
    let pos1 = Position::from_start(input1);
    let pos2 = Position::from_start(input2);

    let _ = pos1.cmp(&pos2);
}

#[test]
#[cfg(feature = "std")]
fn hash() {
    use std::collections::HashSet;

    let input = "a";
    let start = Position::from_start(input);
    let mut positions = HashSet::new();

    positions.insert(start);
}
