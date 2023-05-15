use crate::{
    data::SymbolAlias,
    rule::{FunctionRule, GrammarRule, YggdrasilIdentifier},
};

use indexmap::IndexMap;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use url::Url;

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrammarInfo {
    /// File path of the grammar
    pub url: Option<Url>,
    pub name: YggdrasilIdentifier,
    pub extensions: Vec<String>,
    pub imports: BTreeMap<Url, Vec<SymbolAlias>>,
    pub exports: Vec<String>,
    pub rules: IndexMap<String, GrammarRule>,
    /// Named set of rules
    ///
    /// ```ygg
    /// @style(keyword)
    /// token Keywords {
    ///     KW_FOR: 'for'
    ///     KW_IN: 'in'
    /// }
    /// ```
    /// Warning: this collection cannot be indexed externally without naming
    pub token_sets: IndexMap<String, Vec<YggdrasilIdentifier>>,
    pub functions: IndexMap<String, FunctionRule>,
}

impl Default for GrammarInfo {
    fn default() -> Self {
        Self {
            url: None,
            name: Default::default(),
            extensions: vec![],
            imports: Default::default(),
            exports: vec![],
            rules: Default::default(),
            token_sets: Default::default(),
            functions: Default::default(),
        }
    }
}

impl GrammarInfo {
    pub fn parser_name(&self) -> String {
        format!("{}Parser", self.name.text)
    }
    pub fn rules(&self) -> Vec<GrammarRule> {
        self.rules.values().cloned().collect()
    }
    pub fn ignored_rules(&self) -> Vec<GrammarRule> {
        todo!()
    }
    pub fn insert(&mut self, rule: GrammarRule) -> Option<GrammarRule> {
        let key = rule.name.text.clone();
        self.rules.insert(key, rule)
    }
}
