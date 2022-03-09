use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use yggdrasil_error::Url;

use crate::*;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GrammarInfo {
    /// File path of the grammar
    pub url: Option<Url>,
    pub name: String,
    pub extensions: Vec<String>,
    pub imports: BTreeMap<Url, Vec<SymbolAlias>>,
    pub exports: Vec<String>,
    pub rules: BTreeMap<String, GrammarRule>,
    pub functions: Vec<String>,
    pub rule_prefix: String,
    pub rule_suffix: String,
}

impl Default for GrammarInfo {
    fn default() -> Self {
        Self {
            url: None,
            name: "".to_string(),
            extensions: vec![],
            imports: Default::default(),
            exports: vec![],
            rules: Default::default(),
            functions: vec![],
            rule_prefix: "".to_string(),
            rule_suffix: "Node".to_string(),
        }
    }
}

impl GrammarInfo {
    pub fn ignored_rules(&self) -> Vec<GrammarRule> {
        todo!()
    }
}
