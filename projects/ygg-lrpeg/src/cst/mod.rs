use std::{collections::HashMap, fmt::Write};

// mod write_anonymous;
// mod write_header;
mod write_mod;
mod write_parse;
mod write_rule;

pub trait PestCST {
    fn write_ast(&self) -> std::fmt::Result;
    fn write_cst(&self) -> std::fmt::Result;
}

pub struct PestCSTW {
    rules: Vec<String>,
    ignores: Vec<String>,
}
