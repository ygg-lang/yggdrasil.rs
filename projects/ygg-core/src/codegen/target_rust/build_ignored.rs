use std::ops::Range;

use askama::Template;
use convert_case::{Case, Casing};
use yggdrasil_ir::{
    grammar::GrammarInfo,
    nodes::{ExpressionKind, YggdrasilExpression},
    rule::{GrammarRule, GrammarRuleKind},
};

use crate::codegen::RustCodegen;

#[derive(Template)]
#[template(path = "rust/union_consume.djv", escape = "none")]
struct UnionConsume {
    derive_debug: bool,
    name_upper: String,
    name_lower: String,
    node_name: String,
    branches: Vec<UnionBranchItem>,
}

struct UnionBranchItem {
    target_node: String,
    consume_lower: String,
    branch_upper: String,
    branch_lower: String,
}

#[derive(Template)]
#[template(path = "rust/ignored_consume.djv", escape = "none")]
struct IgnoreItems {
    items: Vec<IgnoredItem>,
}

struct IgnoredItem {
    class: String,
    consume: String,
}

impl RustCodegen {
    fn ignored_rules(&self, info: &GrammarInfo) -> IgnoreItems {
        IgnoreItems {
            items: info
                .ignores
                .iter()
                .map(|name| IgnoredItem { class: self.node_name(name), consume: self.consume_name(name) })
                .collect(),
        }
    }
    fn consume_union(&self, rule: &GrammarRule) -> UnionConsume {
        assert_eq!(rule.kind, GrammarRuleKind::Union);
        let mut branches = vec![];
        for (name, node) in rule.get_branches() {
            let target_node = match node.as_rule() {
                Some(s) => self.node_name(&s.name.text),
                None => unreachable!("Must build proxy node first"),
            };
            branches.push(UnionBranchItem {
                target_node,
                consume_lower: format!("consume_{name}").to_case(Case::Snake),
                branch_upper: name.to_case(Case::Pascal),
                branch_lower: name.to_case(Case::Snake),
            })
        }
        UnionConsume {
            derive_debug: true,
            name_upper: self.upper_name(&rule.name.text),
            name_lower: self.lower_name(&rule.name.text),
            node_name: self.node_name(&rule.name.text),
            branches,
        }
    }
}

impl RustCodegen {
    pub fn node_name(&self, name: &str) -> String {
        let name = format!("{}_{}_{}", self.rule_prefix, name, self.rule_suffix);
        name.to_case(Case::Pascal)
    }
    /// never be a keyword
    pub fn consume_name(&self, name: &str) -> String {
        let name = format!("consume_{}", name);
        name.to_case(Case::Snake)
    }
    pub fn upper_name(&self, name: &str) -> String {
        name.to_case(Case::Pascal)
    }
    pub fn lower_name(&self, name: &str) -> String {
        self.prevent_keyword(name.to_case(Case::Snake))
    }
    fn prevent_keyword(&self, mut name: String) -> String {
        todo!()
    }
}

#[test]
fn test() {
    let mut info = GrammarInfo::default();
    info.ignores.insert("Whitespace".to_string());
    info.ignores.insert("CommentLine".to_string());
    let codegen = RustCodegen::default();
    let items = codegen.ignored_rules(&info);
    let result = items.render().unwrap();
    println!("{}", result);
}
