use std::ops::Range;

use askama::Template;
use convert_case::{Case, Casing};

use yggdrasil_ir::{DataKind, ExpressionKind, ExpressionNode, GrammarInfo, GrammarRule, GrammarRuleKind, RuleReference};

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
                Some(s) => self.node_name(&s.name),
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
            derive_debug: rule.derives.debug,
            name_upper: self.upper_name(&rule.name),
            name_lower: self.lower_name(&rule.name),
            node_name: self.node_name(&rule.name),
            branches,
        }
    }
    fn do_parse(&self, node: &ExpressionNode, state: &str, target: &str) -> String {
        match &node.kind {
            ExpressionKind::Function(_) => {
                unreachable!()
            }
            ExpressionKind::Choice(_) => {
                unreachable!()
            }
            ExpressionKind::Concat(_) => {
                unreachable!()
            }
            ExpressionKind::Unary(_) => {
                unreachable!()
            }
            ExpressionKind::Rule(_) => {
                unreachable!()
            }
            ExpressionKind::Data(v) => match &**v {
                DataKind::Ignored => {
                    format!("let ({state}, _) = {ignored}::consume({state})?;", ignored = self.node_name("Ignored"))
                }
                DataKind::Boolean(v) => {
                    // log::warn!("bool value `{}` can only be used in macro calls", v)
                    format!("let ({state}, {target}) = {state}.match_string({v:?})?;", v = v.to_string())
                }
                DataKind::Integer(v) => {
                    // unreachable!("integer value `{}` can only be used in macro calls", v)
                    format!("let ({state}, {target}) = {state}.match_string({v:?})?;", v = v.to_string())
                }
                DataKind::String(s) => {
                    format!("let ({state}, {target}) = {state}.match_string({s:?})?;", s = s)
                }
                DataKind::StringFused(_) => {
                    unimplemented!()
                }
                DataKind::CharacterAny => {
                    format!("let ({state}, {target}) = {state}.match_char_any()?;")
                }
                DataKind::Character(c) => {
                    format!("let ({state}, {target}) = {state}.match_char({c:?})?;", c = c)
                }
                DataKind::CharacterBuiltin(_) => {
                    unreachable!()
                }
                DataKind::CharacterRange(c) => {
                    format!("let ({state}, {target}) = {state}.match_char_range({s:?}, {e:?})?;", s = c.start(), e = c.end())
                }
                DataKind::CharacterFused(_) => {
                    unimplemented!()
                }
            },
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
        let keywords = &[
            "abstract", "alignof", "as", "become", "box", "break", "const", "continue", "crate", "do", "else", "enum",
            "extern", "false", "final", "fn", "for", "if", "impl", "in", "let", "loop", "macro", "match", "mod", "move", "mut",
            "offsetof", "override", "priv", "proc", "pub", "pure", "ref", "return", "Self", "self", "sizeof", "static",
            "struct", "super", "trait", "true", "type", "typeof", "unsafe", "unsized", "use", "virtual", "where", "while",
            "yield",
        ];
        if keywords.contains(&name.as_str()) { format!("r#{name}") } else { name }
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
    let codegen = RustCodegen::default();
    let mut rule = GrammarRule::new("Statement", &Range::default(), GrammarRuleKind::Union);
    let expr = RuleReference::new("ClassStatement").to_node("Class") | RuleReference::new("UnionStatement").to_node("Union");
    rule.body = expr;
    let items = codegen.consume_union(&rule);
    let result = items.render().unwrap();
    println!("{}", result);
}
