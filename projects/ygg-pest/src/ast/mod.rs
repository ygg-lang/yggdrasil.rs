mod nodes;
mod parse;

use crate::CSTBuilder;
use crate::Rule;
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use yggdrasil_cst_shared::position_system::OffsetRange;

pub use self::nodes::*;

/// ASTResult
pub type ASTResult<T> = Result<T, Error<Rule>>;

/// ASTBuilder
pub struct ASTBuilder {
    /// errors
    errors: Vec<Error<Rule>>,
}

impl ASTBuilder {
    /// parse_program
    pub fn parse_program(input: &str) -> ASTResult<Program> {
        let parsed = CSTBuilder::parse(Rule::program, input)?;
        let program;
        match parsed.into_iter().next() {
            None => {
                unimplemented!()
            }
            Some(p) => Program::try_one(pair),
        }
    }
}

impl Program {
    pub fn try_many() {}
    pub fn try_some() {}
    pub fn try_one() {}
}
