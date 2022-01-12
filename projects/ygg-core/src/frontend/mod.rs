use std::{collections::BTreeMap, ops::Range};

use crate::{frontend::rule::ExpressionKind, YggdrasilError};
use indexmap::map::IndexMap;
use lsp_types::Url;
use yggdrasil_bootstrap::Result;

// mod optimize;
pub mod rule;
pub mod typing;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GrammarInfo {
    /// File path of the grammar
    pub url: Option<Url>,
    pub name: Symbol,
    pub extensions: Vec<Symbol>,
    pub ignores: Vec<Symbol>,
    pub imports: BTreeMap<Url, Vec<SymbolAlias>>,
    pub exports: Vec<String>,
    pub rules: BTreeMap<String, GrammarRule>,
    pub macros: Vec<String>,
    pub rule_prefix: String,
    pub rule_suffix: String,
}

pub trait CodeOptimizer {
    fn optimize(&self, info: &mut GrammarInfo) -> Result<Diagnostics<()>>;
}

pub trait CodeGenerator<T> {
    fn generate(&self, info: &GrammarInfo) -> Result<Diagnostics<T>>;
}

impl GrammarInfo {
    pub fn optimize(&mut self, pass: &[impl CodeOptimizer]) -> Result<Diagnostics<()>> {
        let mut errors = vec![];
        for opt in pass {
            errors.extend(opt.optimize(self)?.errors);
        }
        Ok(Diagnostics { success: (), errors })
    }
    pub fn codegen<T>(&self, pass: impl CodeGenerator<T>) -> Result<Diagnostics<T>> {
        pass.generate(self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub range: Range<usize>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SymbolAlias {
    pub name: String,
    pub alias: String,
    pub range: Range<usize>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GrammarRule {
    /// Automatically inline when this rule is called
    ///
    /// ## Examples
    /// ```ygg
    /// def RuleName {
    ///
    /// }
    /// ```
    pub name: String,
    /// Automatically inline when this rule is called
    ///
    /// ## Examples
    /// ```ygg
    /// def rule -> char {
    ///
    /// }
    ///
    /// def rule() -> char {
    ///
    /// }
    /// ```
    pub r#type: String,
    /// Document of this rule
    ///
    /// ## Examples
    /// ```ygg
    /// 
    /// def rule {
    ///
    /// }
    ///
    /// def rule() -> char {
    ///
    /// }
    /// ```
    pub document: String,
    ///
    pub derives: RuleDerive,
    /// Automatically inline when this rule is called
    ///
    /// ## Examples
    /// ```ygg
    /// #inline(true)
    /// def rule {
    ///
    /// }
    ///
    /// def inline rule {
    ///
    /// }
    ///
    /// def _rule {
    ///
    /// }
    /// ```
    pub auto_inline: bool,
    /// Automatically box when this rule is called
    ///
    /// ## Examples
    /// ```ygg
    /// #boxed(true)
    /// def rule {
    ///
    /// }
    ///
    /// def boxed rule {
    ///
    /// }
    /// ```
    pub auto_boxed: bool,
    pub auto_capture: bool,
    pub atomic_rule: bool,
    ///
    pub body: ExpressionKind,
    /// position of all parts
    pub range: Range<usize>,
}

#[derive(Clone, Eq, PartialEq)]
pub struct RuleDerive {
    pub(crate) parser: Option<String>,
    pub(crate) debug: Option<String>,
    pub(crate) display: Option<String>,
    pub(crate) eq: bool,
    pub(crate) eq_partial: Option<String>,
    pub(crate) ord: bool,
    pub(crate) ord_partial: Option<String>,
    pub(crate) hash: Option<String>,
}

impl Default for RuleDerive {
    fn default() -> Self {
        Self {
            parser: None,
            debug: None,
            display: None,
            eq: false,
            eq_partial: None,
            ord: false,
            ord_partial: None,
            hash: None,
        }
    }
}
