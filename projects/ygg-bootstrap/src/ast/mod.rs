mod nodes;
mod parse;

use crate::cst::CSTBuilder;
use crate::cst::Rule;
use crate::{Error, Result};
use pest::iterators::Pair;
use pest::Parser;
use std::collections::HashMap;
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
    fn many(pairs: Vec<Pair<Rule>>, errors: &mut Vec<Error>) -> Vec<Self> {
        let mut out = Vec::with_capacity(pairs.len());
        for pair in pairs {
            match Self::parse_pair(pair, errors) {
                Ok(o) => out.push(o),
                Err(Error::Unwinding) => (),
                Err(e) => errors.push(e),
            }
        }
        return out;
    }
    fn some(pairs: Pair<Rule>, buffer: &mut Option<Self>, errors: &mut Vec<Error>) {
        match Self::parse_pair(pairs, errors) {
            Ok(o) => *buffer = Some(o),
            Err(Error::Unwinding) => (),
            Err(e) => errors.push(e),
        }
    }
    fn try_one(pairs: Pair<Rule>, buffer: &mut Option<Self>, errors: &mut Vec<Error>) -> Result<()> {
        match Self::parse_pair(pairs, errors) {
            Ok(o) => Ok(*buffer = Some(o)),
            Err(Error::Unwinding) => Err(Error::Unwinding),
            Err(e) => {
                errors.push(e);
                Err(Error::Unwinding)
            }
        }
    }

    fn named_many(map: &mut HashMap<String, Vec<Pair<Rule>>>, tag: &str, errors: &mut Vec<Error>) -> Vec<Self> {
        let pairs = match map.remove(tag) {
            Some(s) => s,
            _ => return vec![],
        };
        return Self::many(pairs, errors);
    }

    fn named_some(map: &mut HashMap<String, Vec<Pair<Rule>>>, tag: &str, errors: &mut Vec<Error>) -> Option<Self> {
        let pair = match map.remove(tag).as_mut().map(|v| v.remove(0)) {
            Some(s) => s,
            _ => return None,
        };
        match Self::parse_pair(pair, errors) {
            Ok(o) => Some(o),
            Err(Error::Unwinding) => None,
            Err(e) => {
                errors.push(e);
                None
            }
        }
    }

    fn try_named_one(map: &mut HashMap<String, Vec<Pair<Rule>>>, tag: &str, errors: &mut Vec<Error>) -> Result<Self> {
        let pair = match map.remove(tag).as_mut().map(|v| v.remove(0)) {
            Some(s) => s,
            _ => return Err(Error::node_missing(tag)),
        };
        match Self::parse_pair(pair, errors) {
            Ok(o) => Ok(o),
            Err(Error::Unwinding) => Err(Error::Unwinding),
            Err(e) => {
                errors.push(e);
                Err(Error::Unwinding)
            }
        }
    }

    fn parse_pair(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self>;
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
        let program = Program::parse_pair(pairs, &mut self.errors)?;
        Ok(program)
    }
}
