mod nodes;
mod parse;

use std::collections::HashMap;
use crate::cst::{CSTBuilder, Node};
use yggdrasil_shared::records::get_position;
use yggdrasil_shared::records::LSPRange as Range;
use yggdrasil_shared::traits::{ASTNode, CSTNode};
use yggdrasil_shared::Error;
use yggdrasil_shared::Result;

pub use self::nodes::*;

/// ASTBuilder
pub struct ASTBuilder {
    cst: CSTBuilder,
    /// input text
    pub input: String,
    /// errors report in this parsing
    pub errors: Vec<Error>,
}

impl Default for ASTBuilder {
    fn default() -> Self {
        Self {
            cst: Default::default(),
            input: String::new(),
            errors: vec![],
        }
    }
}

impl CSTNode for Node {
    fn get_str(&self) -> &str {
        todo!()
    }

    fn get_span(&self) -> (usize, usize) {
        (self.start, self.end)
    }

    fn get_range(&self) -> (usize, usize, usize, usize) {
        todo!()
    }

    fn get_node_tag(&self) -> Option<&'static str> {
        self.label
    }

    fn get_branch_tag(&self) -> Option<&'static str> {
        self.alternative
    }

    fn get_tag_map(&self) -> HashMap<&'static str, Vec<Self>> {
        let mut out: HashMap<&'static str, Vec<Self>> = HashMap::new();
        for node in self.children {
            if let Some(s) = node.get_node_tag() {
                match out.get_mut(s) {
                    Some(s) => s.push(pair),
                    None => {
                        out.insert(s, vec![pair]);
                    }
                }
            }
        }
        return out;
    }
}

impl ASTBuilder {
    /// parse_program
    pub fn parse_program(&mut self, input: &str) -> Result<Program> {
        self.errors.clear();
        self.input = String::from(input);
        let cst = self.cst.parse(&self.input)?;
        let program = Program::parse(cst, &mut self.errors)?;
        Ok(program)
    }
}
