use tree_sitter_cli::generate::grammars::{Variable, VariableType};
use tree_sitter_cli::generate::rules::Rule;

mod from_ast;
mod expression;
pub use self::expression::*;

pub type Map<K, V> = std::collections::HashMap<K, V>;
// pub type Map<K, V> = indexmap::IndexMap<K, V>;

#[derive(Clone, Debug)]
pub struct YGGRule {
    ///
    name: String,
    ///
    structure_name: Option<String>,
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
    ///
    expression: RefinedExpression
}

#[derive(Clone, Debug)]
pub struct MetaInfo {
    name: String,
    exts: Vec<String>,
}



#[derive(Clone, Debug)]
pub struct RuleMap {
    inner: Map<String, YGGRule>,
}

impl RuleMap {
    pub fn optimize(&mut self) {
        self.merge_regex();
        self.inline()
    }
    fn inline(&mut self) {

    }
    fn merge_regex(&mut self) {

    }
    pub fn named_rules(&self) -> Vec<YGGRule> {
        self.inner.values().cloned().filter(|r| !r.force_inline).collect()
    }
}

impl YGGRule {
    fn inline(&mut self, map: &RuleMap) {

    }
    fn merge_regex(&mut self) {

    }
}

impl YGGRule {
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


impl YGGRule {
    pub fn build_variable(&self) -> Variable {
        Variable {
            name: self.name.to_owned(),
            kind: VariableType::Named,
            rule: Rule::Blank
        }
    }
}