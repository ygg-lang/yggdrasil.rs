use std::{fs::File, io::prelude::*, path::Path};
use ygg_from_antlr4::{flatten, PEG};

#[test]
fn ready() {
    println!("it, works!")
}


pub fn peg_assert(input: &str, target: &str) {
    let mut p = PEG::new();
    let out = flatten(p.parse(input).unwrap());
    assert_eq!(format!("{:#?}", out), target)
}

#[test]
fn grammar() {
    let input = include_str!("csv.g4");
    peg_assert(input, include_str!("csv.ygg"))
}
