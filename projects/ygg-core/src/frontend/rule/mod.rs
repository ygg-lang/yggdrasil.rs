mod from_ast;
// mod hints;
mod node;

pub use self::node::{
    choice::ChoiceExpression, concat::ConcatExpression, data::DataKind, unary::UnaryExpression, ExpressionKind, ExpressionNode,
};
use crate::frontend::{GrammarInfo, Symbol};

impl Default for GrammarInfo {
    fn default() -> Self {
        Self {
            url: None,
            is_grammar: false,
            name: Symbol { name: "".to_string(), range: Default::default() },
            extensions: vec![],
            ignores: vec![],
            imports: Default::default(),
            rules: Default::default(),
            rule_prefix: "".to_string(),
            rule_suffix: "Node".to_string(),
        }
    }
}

// impl GrammarInfo {
//     #[inline]
//     pub fn get(&self, rule: &str) -> Option<&Rule> {
//         self.rule_map.get(rule)
//     }
//     pub fn get_inline(&self, rule: &str) -> Option<&Rule> {
//         match self.rule_map.get(rule) {
//             Some(s) => Some(s),
//             // Check manual inlining
//             None if rule.starts_with("_") => self.rule_map.get(&rule[1..=rule.len()]),
//             _ => None,
//         }
//     }
//     #[inline]
//     pub fn get_mut(&mut self, rule: &str) -> Option<&mut Rule> {
//         self.rule_map.get_mut(rule)
//     }
//     #[inline]
//     pub fn get_inline_mut(&mut self, rule: &str) -> Option<&mut Rule> {
//         match self.rule_map.get(rule) {
//             Some(_) => self.rule_map.get_mut(rule),
//             // Check manual inlining
//             None if rule.starts_with("_") => self.rule_map.get_mut(&rule[1..=rule.len()]),
//             _ => None,
//         }
//     }
//     #[inline]
//     pub fn insert(&mut self, name: String, rule: Rule) -> Option<Rule> {
//         self.rule_map.insert(name, rule)
//     }
//     #[inline]
//     pub fn keys(&self) -> Keys<String, Rule> {
//         self.rule_map.keys()
//     }
//     #[inline]
//     pub fn rules(&self) -> Values<String, Rule> {
//         self.rule_map.values()
//     }
//     #[inline]
//     pub fn named_rules(&self) -> Vec<Rule> {
//         self.rule_map.values().cloned().filter(|r| !r.auto_inline).collect()
//     }
// }
//
// impl Rule {
//     pub fn build_structure(&self) -> String {
//         unimplemented!()
//     }
//     pub fn build_parse(&self) -> String {
//         unimplemented!()
//     }
//     pub fn build_error(&self) -> String {
//         unimplemented!()
//     }
// }
