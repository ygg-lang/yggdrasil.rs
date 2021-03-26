mod from_ast;

use std::collections::HashMap;

pub type Map<K, V> = std::collections::HashMap<K, V>;
// pub type Map<K, V> = indexmap::IndexMap<K, V>;

#[derive(Clone,Debug)]
pub struct Rule {
    ///
    name: String,
    ///
    structure: Option<String>,
    ///
    force_inline: bool,
    /// Eliminate unnamed nodes
    /// ```ygg
    /// name <- expr
    /// ^expr
    /// ```
    eliminate_unmarked: bool,
    /// Eliminate unnamed nodes
    /// ```ygg
    /// "string"
    /// /regex/
    /// [0-9a-z]
    /// 012345
    /// ```
    eliminate_unnamed: bool,
}

#[derive(Clone,Debug)]
pub struct MetaInfo {
    name: String,
    exts: Vec<String>,
}

#[derive(Clone,Debug)]
pub struct RuleMap {
    inner: Map<String, Rule>,
}

impl RuleMap {
    pub fn inline(&mut self) {

    }
    pub fn named_rules(&self) -> Vec<Rule> {
        self.inner.values().cloned().filter(|r| !r.force_inline).collect()
    }
}

impl Rule {
    pub fn build_structure(&self) -> String {
        unimplemented!()
    }
    pub fn build_parse(&self) -> String {
        unimplemented!()
    }
    pub fn build_error(&self) -> String {
        unimplemented!()
    }
}