mod nodes;
mod parse;

use std::collections::HashMap;
use crate::cst::{Node, PEG};
use yggdrasil_shared::records::{CSTBuilder,ASTBuilder};
use yggdrasil_shared::traits::{ASTNode, CSTNode};
use yggdrasil_shared::Error;
use yggdrasil_shared::Result;

pub use self::nodes::*;

impl CSTNode for Node {
    fn get_string(&self, input:&str) -> String {
        unsafe {
            input.get_unchecked(self.start..self.end).to_string()
        }
    }

    fn get_span(&self) -> (usize, usize) {
        (self.start, self.end)
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
                    Some(s) => s.push(node),
                    None => {
                        out.insert(s, vec![node]);
                    }
                }
            }
        }
        return out;
    }
}

pub struct Ygg {
    peg: PEG,
    cst: CSTBuilder,
    ast: ASTBuilder,
}

impl Default for Ygg {
    fn default() -> Self {
        Self {
            peg: PEG::new(),
            cst: Default::default(),
            ast: Default::default()
        }
    }
}

impl Ygg {
    /// parse_program
    pub fn parse_program(&mut self, input: &str) -> Result<(Program, Vec<Error>)> {
        self.init(input);
        let cst = self.cst.parse(&self.ast.input)?;
        let program = Program::parse(cst, &mut self.ast)?;
        let mut error = Vec::with_capacity(self.ast.error.len() + self.cst.error.len());
        error.extend()


        Ok((program))
    }
    fn init(&mut self, input: &str) {
        self.cst.error.clear();
        self.ast.error.clear();
        self.ast.input = String::from(input)
    }
}
