mod nodes;

// mod parse;

// use crate::cst::CSTBuilder;
// use crate::cst::Rule;
use yggdrasil_shared::{Error, Result};
use pest::iterators::Pair;
use pest::Parser;
use std::collections::HashMap;
use yggdrasil_shared::records::{get_position, LSPRange as Range};
use yggdrasil_shared::traits::ASTNode;

pub use self::nodes::*;

/// ASTBuilder
pub struct ASTBuilder {
    /// errors report in this parsing
    pub errors: Vec<Error>,
}

impl Default for ASTBuilder {
    fn default() -> Self {
        Self { errors: vec![] }
    }
}
/*
impl ASTBuilder {
    /// parse_program
    pub fn parse_program(&mut self, input: &str) -> Result<Program> {
        self.errors.clear();
        let parsed = CSTBuilder::parse(Rule::program, input)?;
        let pairs = parsed.into_iter().next().ok_or(Error::node_missing("program not found"))?;
        let program = Program::parse(pairs, &mut self.errors)?;
        Ok(program)
    }
}
*/