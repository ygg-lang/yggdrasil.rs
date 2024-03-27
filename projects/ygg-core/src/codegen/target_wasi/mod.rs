use crate::{codegen::BuildRailway, parse_grammar, FileCache};
use askama::Template;
use heck::{ToKebabCase, ToSnakeCase, ToUpperCamelCase};
use itertools::Itertools;
use railroad::{Diagram, Node, VerticalGrid};
use std::{
    fmt::Write,
    fs::{create_dir_all, File},
    io::{Error, ErrorKind, Write as _},
    path::{Path, PathBuf},
};
use yggdrasil_error::{FileID, Success, Validate, Validation};
use yggdrasil_ir::{
    grammar::GrammarInfo,
    rule::GrammarRule,
    traits::{CodeGenerator, CodeOptimizer},
};

mod build_ast;
mod build_cst;
mod build_main;
mod build_readme;
mod build_wit;
mod filters;
mod grammar_ext;
mod rule_ext;

use self::{grammar_ext::GrammarExt, rule_ext::RuleExt};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BuildWasi {
    pub export: String,
    pub railway: BuildRailway,
}

impl Default for BuildWasi {
    fn default() -> Self {
        Self { export: "src".to_string(), railway: Default::default() }
    }
}

#[derive(Template)]
#[template(path = "wasi/main.jinja", escape = "none")]
pub struct RustWriteMain<'i> {
    grammar: &'i GrammarInfo,
    config: BuildWasi,
}

#[derive(Template)]
#[template(path = "wasi/wit.jinja", escape = "none")]
pub struct WasiWriteWit<'i> {
    grammar: &'i GrammarInfo,
    config: BuildWasi,
}

#[derive(Template)]
#[template(path = "wasi/cst.jinja", escape = "none")]
pub struct RustWriteCST<'i> {
    grammar: &'i GrammarInfo,
    config: BuildWasi,
}

#[derive(Template)]
#[template(path = "wasi/ast.jinja", escape = "none")]
pub struct RustWriteAST<'i> {
    grammar: &'i GrammarInfo,
    config: BuildWasi,
}

#[derive(Template)]
#[template(path = "wasi/readme.jinja", escape = "none")]
pub struct RustWriteReadme<'i> {
    grammar: &'i GrammarInfo,
    config: BuildWasi,
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
    pub ron: String,
}

impl CodeGenerator for BuildWasi {
    type Output = RustModule;

    fn generate(&mut self, info: &GrammarInfo) -> Validation<Self::Output> {
        let mut out = RustModule::default();
        let mut errors = vec![];
        out.ron = format!("{:#?}", info);
        out.main = RustWriteMain { grammar: info, config: self.clone() }.render().recover(&mut errors)?;
        out.lex = WasiWriteWit { grammar: info, config: self.clone() }.render().recover(&mut errors)?;
        out.cst = RustWriteCST { grammar: info, config: self.clone() }.render().recover(&mut errors)?;
        out.ast = RustWriteAST { grammar: info, config: self.clone() }.render().recover(&mut errors)?;
        let readme = RustWriteReadme {
            grammar: info,
            config: self.clone(),
            railroad: self.railway.generate(info).recover(&mut errors)?,
        };
        out.readme = readme.render().recover(&mut errors)?;
        out.railway = readme.railway_svg();
        out.railway_min = readme.railway_min();
        Success { value: out, diagnostics: errors }
    }
}

impl BuildWasi {
    pub fn generate<P: AsRef<Path>>(&self, id: FileID, cache: &mut FileCache, output: P) -> Validation<PathBuf> {
        let mut errors = vec![];
        let info = parse_grammar(id, cache).validate(&mut errors)?;
        let out = info.generate(self.clone()).validate(&mut errors)?;
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
        let mut main = File::create(path.join("src/wit/mod.rs"))?;
        main.write_all(self.main.as_bytes())?;
        let mut cst = File::create(path.join("wit/world.wit"))?;
        cst.write_all(self.lex.as_bytes())?;
        let mut cst = File::create(path.join("src/wit/parse_cst.rs"))?;
        cst.write_all(self.cst.as_bytes())?;
        let mut ast = File::create(path.join("src/wit/parse_ast.rs"))?;
        ast.write_all(self.ast.as_bytes())?;
        if !self.railway.is_empty() {
            let mut ast = File::create(path.join("src/wit/railway.svg"))?;
            ast.write_all(self.railway.as_bytes())?;
            let mut ast = File::create(path.join("src/wit/railway.min.svg"))?;
            ast.write_all(self.railway_min.as_bytes())?;
        }
        let mut ast = File::create(path.join("readme.md"))?;
        ast.write_all(self.readme.as_bytes())?;
        let mut ast = File::create(path.join("debug.ron"))?;
        ast.write_all(self.ron.as_bytes())?;
        path.canonicalize()
    }
}
