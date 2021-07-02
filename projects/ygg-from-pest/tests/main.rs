#[macro_use]
extern crate quote;
extern crate pest_generator;
extern crate proc_macro;
use pest_generator::derive_parser;
use std::{fs::File, io::prelude::*, path::Path};

#[test]
fn ready() {
    println!("it, works!")
}
