use super::*;
use alloc::{borrow::ToOwned, vec::Vec};

#[test]
fn split() {
    let input = "a";
    let start = position::Position::from_start(input);
    let mut end = start.clone();

    assert!(end.skip(1));

    let span = start.clone().span(&end.clone());

    assert_eq!(span.split(), (start, end));
}

#[test]
fn lines_mid() {
    let input = "abc\ndef\nghi";
    let span = Span::new(input, 1, 7).unwrap();
    let lines: Vec<_> = span.lines().collect();
    // println!("{:?}", lines);
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "abc\n".to_owned());
    assert_eq!(lines[1], "def\n".to_owned());
}

#[test]
fn lines_eof() {
    let input = "abc\ndef\nghi";
    let span = Span::new(input, 5, 11).unwrap();
    assert!(span.end_pos().at_end());
    let lines: Vec<_> = span.lines().collect();
    // println!("{:?}", lines);
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "def\n".to_owned());
    assert_eq!(lines[1], "ghi".to_owned());
}
