use askama::Template;
use fs::read_to_string;
use itertools::Itertools;
use std::{
    collections::{btree_map::Values, BTreeMap},
    fmt::{Arguments, Write},
    fs,
    fs::File,
    io::Write as _,
    path::Path,
};
use yggdrasil_ir::{grammar::GrammarInfo, rule::GrammarRule, traits::CodeGenerator, QError, QResult, Validation};

mod build_data;
mod build_ignored;
mod grammar_ext;
mod rule_ext;

use self::rule_ext::RuleExt;

pub struct RustCodegen {
    buffer: String,
    enable_position: bool,
    rule_prefix: String,
    rule_suffix: String,
}

impl Default for RustCodegen {
    fn default() -> Self {
        Self { buffer: "".to_string(), enable_position: true, rule_prefix: "".to_string(), rule_suffix: "Node".to_string() }
    }
}

impl RustCodegen {
    pub fn codegen(&mut self, src: impl AsRef<Path>) -> QResult {
        self.buffer.clear();
        let path = src.as_ref().to_path_buf().canonicalize()?;
        let dir = match path.parent() {
            Some(s) => s,
            None => return Err(QError::runtime_error("ygg dir not found")),
        };
        let mut peg = File::create(path.with_extension("ebnf"))?;
        let text = read_to_string(&path)?;
        // let info = match GrammarParser::parse(&text) {
        //     Validation::Success { value, diagnostics } => value,
        //     Validation::Failure { fatal, diagnostics } => return Err(fatal),
        // };
        // let tokens = match self.generate(&info) {
        //     Validation::Success { value, diagnostics } => value,
        //     Validation::Failure { fatal, diagnostics } => return Err(fatal),
        // };
        todo!();
        // write!(peg, "{}", self.buffer)?;
        // Ok(())
    }
}

#[derive(Template)]
#[template(path = "rust/main.jinja", escape = "none")]
pub struct RustWrite<'i> {
    grammar: &'i GrammarInfo,
}

impl CodeGenerator for RustCodegen {
    type Output = String;

    fn generate(&mut self, info: &GrammarInfo) -> Validation<Self::Output> {
        let inner = RustWrite { grammar: info };
        Validation::Success { value: inner.render().unwrap(), diagnostics: vec![] }
    }
}

trait WriteRust {
    fn write_rust(&self, w: &mut RustCodegen, info: &GrammarInfo) -> std::fmt::Result;

    #[track_caller]
    #[allow(unused_variables)]
    fn write_class(&self, w: &mut RustCodegen, info: &GrammarInfo) -> std::fmt::Result {
        unreachable!("should not implement here")
    }
}
