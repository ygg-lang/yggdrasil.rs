mod nodes;
mod parse;

use crate::cst::CSTBuilder;
use crate::cst::Rule;
use crate::{Error, Result};
use pest::iterators::Pair;
use pest::Parser;
use yggdrasil_cst_shared::position_system::{get_position, OffsetRange};

pub use self::nodes::*;

/// ASTBuilder
pub struct ASTBuilder {
    /// errors report in this parsing
    pub errors: Vec<Error>,
}

pub trait ASTParser
where
    Self: Sized,
{
    fn try_many(pairs: Pair<Rule>, buffer: &mut Vec<Self>, errors: &mut Vec<Error>) {
        match Self::parse(pairs, errors) {
            Ok(o) => buffer.push(o),
            Err(Error::Unwinding) => (),
            Err(e) => errors.push(e),
        }
    }
    fn try_some(pairs: Pair<Rule>, buffer: &mut Option<Self>, errors: &mut Vec<Error>) {
        match Self::parse(pairs, errors) {
            Ok(o) => *buffer = Some(o),
            Err(Error::Unwinding) => (),
            Err(e) => errors.push(e),
        }
    }
    fn try_one(pairs: Pair<Rule>, buffer: &mut Option<Self>, errors: &mut Vec<Error>) -> Result<()> {
        match Self::parse(pairs, errors) {
            Ok(o) => Ok(*buffer = Some(o)),
            Err(Error::Unwinding) => Err(Error::Unwinding),
            Err(e) => {
                errors.push(e);
                Err(Error::Unwinding)
            }
        }
    }
    fn parse(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self>;
}

impl Default for ASTBuilder {
    fn default() -> Self {
        Self { errors: vec![] }
    }
}

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
