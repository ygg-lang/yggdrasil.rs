mod nodes;
mod parse;

use crate::cst::{Rule, PEG};
use yggdrasil_shared::records::{ASTBuilder, CSTBuilder};
use yggdrasil_shared::traits::ASTNode;
use yggdrasil_shared::Error;
use yggdrasil_shared::Result;

pub use self::nodes::*;

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
            ast: Default::default(),
        }
    }
}

impl Ygg {
    /// parse_program
    pub fn parse_program(&mut self, input: &str) -> Result<(Program, Vec<Error>)> {
        self.init(input);
        let cst = self.cst.parse(&mut self.peg, &self.ast.input)?;
        let program = Program::parse(cst, &mut self.ast)?;
        let mut error = vec![];
        error.extend(std::mem::take(&mut self.cst.error).into_iter());
        error.extend(std::mem::take(&mut self.ast.error).into_iter());
        Ok((program, error))
    }
    fn init(&mut self, input: &str) {
        self.cst.error.clear();
        self.ast.error.clear();
        self.ast.input = String::from(input)
    }
}
