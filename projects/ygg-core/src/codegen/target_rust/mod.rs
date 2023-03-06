use crate::{
    codegen::Railroad,
    optimize::{InsertIgnore, RefineRules},
};
use askama::Template;
use itertools::Itertools;
#[cfg(feature = "railroad")]
use railroad::{Diagram, Node, VerticalGrid};
use std::{
    fmt::Write,
    fs,
    fs::{create_dir_all, File},
    io::{Error, ErrorKind, Write as _},
    path::{Path, PathBuf},
};
use yggdrasil_error::{Failure, Success, Validate, Validation};
use yggdrasil_ir::{
    grammar::GrammarInfo,
    rule::GrammarRule,
    traits::{CodeGenerator, CodeOptimizer},
};
use yggdrasil_parser::YggdrasilANTLR;

mod build_data;
mod grammar_ext;
mod rule_ext;

use self::{grammar_ext::GrammarExt, rule_ext::RuleExt};

#[derive(Clone, Debug)]
pub struct RustCodegen {
    pub enable_position: bool,
    pub rule_prefix: String,
    pub rule_suffix: String,
}

impl Default for RustCodegen {
    fn default() -> Self {
        Self { enable_position: true, rule_prefix: "".to_string(), rule_suffix: "Node".to_string() }
    }
}

#[derive(Template)]
#[template(path = "rust/main.jinja", escape = "none")]
pub struct RustWriteMain<'i> {
    grammar: &'i GrammarInfo,
    config: RustCodegen,
}

#[derive(Template)]
#[template(path = "rust/lex.jinja", escape = "none")]
pub struct RustWriteLex<'i> {
    grammar: &'i GrammarInfo,
    config: RustCodegen,
}

#[derive(Template)]
#[template(path = "rust/cst.jinja", escape = "none")]
pub struct RustWriteCST<'i> {
    grammar: &'i GrammarInfo,
    config: RustCodegen,
}

#[derive(Template)]
#[template(path = "rust/ast.jinja", escape = "none")]
pub struct RustWriteAST<'i> {
    grammar: &'i GrammarInfo,
    config: RustCodegen,
}

#[derive(Clone, Debug)]
pub struct RustModule {
    pub main: String,
    pub lex: String,
    pub cst: String,
    pub ast: String,
    pub railroad: String,
}

impl CodeGenerator for RustCodegen {
    type Output = RustModule;

    fn generate(&mut self, info: &GrammarInfo) -> Validation<Self::Output> {
        let mut errors = vec![];
        let main = RustWriteMain { grammar: info, config: self.clone() }.render().recover(&mut errors)?;
        let lex = RustWriteLex { grammar: info, config: self.clone() }.render().recover(&mut errors)?;
        let cst = RustWriteCST { grammar: info, config: self.clone() }.render().recover(&mut errors)?;
        let ast = RustWriteAST { grammar: info, config: self.clone() }.render().recover(&mut errors)?;
        let railroad = if cfg!(feature = "railroad") {
            let mut rr = Railroad::default();
            let out: Diagram<VerticalGrid<Box<dyn Node>>> = rr.generate(info).recover(&mut errors)?;
            out.to_string()
        }
        else {
            String::new()
        };
        Success { value: RustModule { main, lex, cst, ast, railroad }, diagnostics: errors }
    }
}

impl RustCodegen {
    pub fn generate<P: AsRef<Path>>(&self, grammar: &str, output: P) -> Validation<PathBuf> {
        let mut errors = vec![];
        let mut info = YggdrasilANTLR::parse(grammar).validate(&mut errors)?;
        info = InsertIgnore::default().optimize(&info).validate(&mut errors)?;
        info = RefineRules::default().optimize(&info).validate(&mut errors)?;
        let out = info.generate(RustCodegen::default()).validate(&mut errors)?;
        out.save(output).validate(&mut errors)
    }
}

impl RustModule {
    pub fn save<P: AsRef<Path>>(&self, path: P) -> std::io::Result<PathBuf> {
        let path = path.as_ref();
        if path.exists() {
            if !path.is_dir() {
                return Err(Error::new(ErrorKind::InvalidInput, "Path is not a directory"));
            }
        }
        else {
            create_dir_all(path)?
        }
        let mut main = File::create(path.join("mod.rs"))?;
        main.write_all(self.main.as_bytes())?;
        let mut cst = File::create(path.join("lexer.rs"))?;
        cst.write_all(self.lex.as_bytes())?;
        let mut cst = File::create(path.join("parse_cst.rs"))?;
        cst.write_all(self.cst.as_bytes())?;
        let mut ast = File::create(path.join("parse_ast.rs"))?;
        ast.write_all(self.ast.as_bytes())?;
        let mut svg = File::create(path.join("railroad.svg"))?;
        svg.write_all(self.railroad.as_bytes())?;
        path.canonicalize()
    }
}
