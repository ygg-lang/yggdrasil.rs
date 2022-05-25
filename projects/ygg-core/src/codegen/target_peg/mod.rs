use fs::read_to_string;
use std::{
    fmt::{Arguments, Write},
    fs,
    fs::File,
    io::Write as _,
    path::Path,
};

use peginator::{
    codegen::{CodegenGrammar, CodegenSettings},
    grammar::Grammar,
    PegParser,
};

use yggdrasil_ir::{
    ChoiceExpression, CodeGenerator, ConcatExpression, DataKind, ExpressionKind, ExpressionNode, FunctionExpression,
    GrammarInfo, GrammarRule, Operator, RuleReference, UnaryExpression, Validation,
};

use crate::parser::GrammarParser;

mod build_class;
mod build_data;
mod build_symbol;

pub struct RustCodegen {
    buffer: String,
}

impl Default for RustCodegen {
    fn default() -> Self {
        Self { buffer: "".to_string() }
    }
}

impl RustCodegen {
    pub fn codegen(&mut self, src: impl AsRef<Path>) -> Result<(), YggdrasilError> {
        self.buffer.clear();
        let path = src.as_ref().to_path_buf().canonicalize()?;
        let dir = match path.parent() {
            Some(s) => s,
            None => return Err(QError::runtime_error("ygg dir not found")),
        };
        let mut peg = File::create(path.with_extension("ebnf"))?;
        let text = read_to_string(&path)?;
        let info = match GrammarParser::parse(&text) {
            Validation::Success { value, diagnostics } => value,
            Validation::Failure { fatal, diagnostics } => return Err(fatal),
        };
        let tokens = match self.generate(&info) {
            Validation::Success { value, diagnostics } => value,
            Validation::Failure { fatal, diagnostics } => return Err(fatal),
        };
        write!(peg, "{}", self.buffer)?;
        Ok(())
    }
}

impl CodeGenerator for RustCodegen {
    type Output = String;

    fn generate(&mut self, info: &GrammarInfo) -> Validation<Self::Output> {}
}

trait WriteRust {
    fn write_rust(&self, w: &mut RustCodegen, info: &GrammarInfo) -> std::fmt::Result;

    #[track_caller]
    #[allow(unused_variables)]
    fn write_class(&self, w: &mut RustCodegen, info: &GrammarInfo) -> std::fmt::Result {
        unreachable!("should not implement here")
    }
}
