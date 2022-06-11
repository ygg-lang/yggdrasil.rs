#![feature(box_syntax)]

pub use crate::parser::parse_grammar;

// noinspection DuplicatedCode
pub mod codegen;
pub mod parser;
