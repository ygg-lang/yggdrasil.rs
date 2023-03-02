use crate::optimize::{InsertIgnore, RefineRules};
use askama::Template;
use fs::read_to_string;
use itertools::Itertools;
use std::{
    collections::{btree_map::Values, BTreeMap},
    fmt::{Arguments, Write},
    fs,
    fs::{create_dir_all, File},
    io::{Error, ErrorKind, Write as _},
    path::{Path, PathBuf},
};
use yggdrasil_error::{Failure, Success, Validation, YggdrasilError};
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
}

impl CodeGenerator for RustCodegen {
    type Output = RustModule;

    fn generate(&mut self, info: &GrammarInfo) -> Validation<Self::Output> {
        let main = RustWriteMain { grammar: info, config: self.clone() }.render().unwrap();
        let lex = RustWriteLex { grammar: info, config: self.clone() }.render().unwrap();
        let cst = RustWriteCST { grammar: info, config: self.clone() }.render().unwrap();
        let ast = RustWriteAST { grammar: info, config: self.clone() }.render().unwrap();
        Success { value: RustModule { main, lex, cst, ast }, diagnostics: vec![] }
    }
}

impl RustCodegen {
    pub fn generate<P: AsRef<Path>>(&self, grammar: &str, output: P) -> Validation<PathBuf> {
        let mut errors = vec![];
        let mut info = match YggdrasilANTLR::parse(grammar) {
            Ok(o) => o,
            Err(e) => return Failure { fatal: YggdrasilError::from(e), diagnostics: errors },
        };
        match InsertIgnore::default().optimize(&info) {
            Success { value, diagnostics } => {
                errors.extend(diagnostics);
                info = value
            }
            Failure { fatal, diagnostics } => {
                errors.extend(diagnostics);
                return Failure { fatal, diagnostics: errors };
            }
        };
        match RefineRules::default().optimize(&info) {
            Success { value, diagnostics } => {
                errors.extend(diagnostics);
                info = value
            }
            Failure { fatal, diagnostics } => {
                errors.extend(diagnostics);
                return Failure { fatal, diagnostics: errors };
            }
        };

        let out = match info.generate(RustCodegen::default()) {
            Success { value, diagnostics } => {
                errors.extend(diagnostics);
                value
            }
            Failure { fatal, diagnostics } => {
                errors.extend(diagnostics);
                return Failure { fatal, diagnostics: errors };
            }
        };
        match out.save(output) {
            Ok(value) => Success { value, diagnostics: errors },
            Err(e) => Failure { fatal: YggdrasilError::from(e), diagnostics: errors },
        }
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
        path.canonicalize()
    }
}
