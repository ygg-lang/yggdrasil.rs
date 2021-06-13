use std::collections::HashMap;
use std::fmt::Write;

// mod write_anonymous;
// mod write_header;
mod write_parse;
mod write_rule;
mod write_mod;

pub trait PestCST {
    fn write_ast(&self) -> std::fmt::Result;
    fn write_cst(&self) -> std::fmt::Result;
}

pub struct PestCSTW {
    rules: Vec<String>,
    ignores: Vec<String>,
}
