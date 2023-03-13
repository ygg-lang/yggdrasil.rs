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

mod build_readme;
mod grammar_ext;
mod rule_ext;

use self::{grammar_ext::GrammarExt, rule_ext::RuleExt};

#[derive(Clone, Debug)]
pub struct RustCodegen {
    pub enable_position: bool,
    pub rule_prefix: String,
    pub rule_suffix: String,
    #[cfg(feature = "railroad")]
    pub railway: Railroad,
}

impl Default for RustCodegen {
    fn default() -> Self {
        Self {
            enable_position: true,
            rule_prefix: "".to_string(),
            rule_suffix: "Node".to_string(),
            #[cfg(feature = "railroad")]
            railway: Default::default(),
        }
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

#[derive(Template)]
#[template(path = "rust/readme.jinja", escape = "none")]
pub struct RustWriteReadme<'i> {
    grammar: &'i GrammarInfo,
    config: RustCodegen,
    #[cfg(feature = "railroad")]
    railroad: Diagram<VerticalGrid<Box<dyn Node>>>,
}

#[derive(Default)]
pub struct RustModule {
    pub main: String,
    pub lex: String,
    pub cst: String,
    pub ast: String,
    pub readme: String,
    pub railway: String,
    pub railway_min: String,
}

impl CodeGenerator for RustCodegen {
    type Output = RustModule;

    fn generate(&mut self, info: &GrammarInfo) -> Validation<Self::Output> {
        let mut out = RustModule::default();
        let mut errors = vec![];
        out.main = RustWriteMain { grammar: info, config: self.clone() }.render().recover(&mut errors)?;
        out.lex = RustWriteLex { grammar: info, config: self.clone() }.render().recover(&mut errors)?;
        out.cst = RustWriteCST { grammar: info, config: self.clone() }.render().recover(&mut errors)?;
        out.ast = RustWriteAST { grammar: info, config: self.clone() }.render().recover(&mut errors)?;
        let readme = RustWriteReadme {
            grammar: info,
            config: self.clone(),
            #[cfg(feature = "railroad")]
            railroad: self.railway.generate(info).recover(&mut errors)?,
        };
        out.readme = readme.render().recover(&mut errors)?;
        out.railway = readme.railway_svg();
        out.railway_min = readme.railway_min();
        Success { value: out, diagnostics: errors }
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
        let mut main = File::create(path.join("annotations"))?;
        main.write_all(self.main.as_bytes())?;
        let mut cst = File::create(path.join("lexer.rs"))?;
        cst.write_all(self.lex.as_bytes())?;
        let mut cst = File::create(path.join("parse_cst.rs"))?;
        cst.write_all(self.cst.as_bytes())?;
        let mut ast = File::create(path.join("parse_ast.rs"))?;
        ast.write_all(self.ast.as_bytes())?;
        if !self.railway.is_empty() {
            let mut ast = File::create(path.join("railway.svg"))?;
            ast.write_all(self.railway.as_bytes())?;
            let mut ast = File::create(path.join("railway.min.svg"))?;
            ast.write_all(self.railway_min.as_bytes())?;
        }
        let mut ast = File::create(path.join("readme.md"))?;
        ast.write_all(self.readme.as_bytes())?;

        path.canonicalize()
    }
}
