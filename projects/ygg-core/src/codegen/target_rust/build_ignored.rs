use std::ops::Range;

use askama::Template;

use yggdrasil_ir::{GrammarInfo, GrammarRule, GrammarRuleKind};

use crate::codegen::RustCodegen;

#[derive(Template)]
#[template(path = "rust/union_consume.djv", escape = "none")]
struct UnionConsume {
    derive_debug: bool,
    name: String,
    node_name: String,
    branches: Vec<UnionBranchItem>,
}

struct UnionBranchItem {
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
                .map(|name| IgnoredItem { class: self.get_class_name(name), consume: self.get_parse_name(name) })
                .collect(),
        }
    }
    fn consume_union(&self, rule: &GrammarRule, info: &GrammarInfo) -> UnionConsume {
        assert_eq!(rule.kind, GrammarRuleKind::Union);
        let mut branches = vec![];
        for (name, _) in rule.get_branches() {}

        UnionConsume {
            derive_debug: rule.derives.debug,
            name: rule.name.clone(),
            node_name: self.get_class_name(&rule.name),
            branches: vec![],
        }
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

#[test]
fn test2() {
    let mut info = GrammarInfo::default();
    info.ignores.insert("Whitespace".to_string());
    info.ignores.insert("CommentLine".to_string());
    let codegen = RustCodegen::default();
    let mut rule = GrammarRule::new("Statement", &Range::default(), GrammarRuleKind::Union);

    let items = codegen.ignored_rules(&info);
    let result = items.render().unwrap();
    println!("{}", result);
}
