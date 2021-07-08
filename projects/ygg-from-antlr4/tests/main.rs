use std::{fs::File, io::prelude::*, path::Path};
use ygg_from_antlr4::{flatten, PEG};

#[test]
fn ready() {
    println!("it, works!")
}

pub fn peg_assert(input: &str) {
    let mut p = PEG::new();
    let out = flatten(p.parse(input).unwrap());
    println!("{:#?}", out)
}

#[test]
fn csv() {
    peg_assert(include_str!("CSV.g4"));
}
#[test]
fn calc() {
    peg_assert(include_str!("Calc.g4"));
}

#[test]
fn rows() {
    peg_assert(include_str!("Rows.g4"));
}
