use crate::{
    data::SymbolAlias,
    nodes::{ChoiceExpression, ExpressionKind, ExpressionNode},
    rule::{FunctionRule, GrammarRule, YggdrasilIdentifier},
    traits::CodeOptimizer,
};
use diagnostic_quick::{QError, QResult, Validation};
use indexmap::IndexMap;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashSet},
    mem::take,
};
use url::Url;

pub use self::insert_ignore::InsertIgnore;

mod auto_tag;
mod dead_code;
mod emit_function;
mod fuse_charset;
mod fuse_rule;
mod inlining;

mod insert_ignore;

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", Serialize, Deserialize)]
pub struct GrammarInfo {
    /// File path of the grammar
    pub url: Option<Url>,
    pub name: YggdrasilIdentifier,
    pub extensions: Vec<String>,
    pub imports: BTreeMap<Url, Vec<SymbolAlias>>,
    pub exports: Vec<String>,
    pub ignores: HashSet<String>,
    pub rules: IndexMap<String, GrammarRule>,
    pub functions: IndexMap<String, FunctionRule>,
    pub range_type: String,
}

impl Default for GrammarInfo {
    fn default() -> Self {
        Self {
            url: None,
            name: Default::default(),
            extensions: vec![],
            imports: Default::default(),
            exports: vec![],
            ignores: Default::default(),
            rules: Default::default(),
            functions: Default::default(),
            range_type: "usize".to_string(),
        }
    }
}

impl GrammarInfo {
    pub fn ignored_rules(&self) -> Vec<GrammarRule> {
        todo!()
    }
    pub fn insert(&mut self, rule: GrammarRule) -> Option<GrammarRule> {
        let key = rule.name.text.clone();
        self.rules.insert(key, rule)
    }
}
