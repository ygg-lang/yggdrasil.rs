use crate::*;
use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    ops::Range,
};

use indexmap::set::IndexSet;

pub mod derive;
pub mod node;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionRule {}

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
    ///
    pub auto_capture: bool,
    ///
    pub atomic_rule: bool,
    ///
    pub body: ExpressionKind,
    /// position of all parts
    pub range: Range<usize>,
}

impl GrammarInfo {
    #[inline]
    pub fn get(&self, rule: &str) -> Option<&Rule> {
        self.rule_map.get(rule)
    }
    pub fn get_inline(&self, rule: &str) -> Option<&Rule> {
        match self.rule_map.get(rule) {
            Some(s) => Some(s),
            // Check manual inlining
            None if rule.starts_with("_") => self.rule_map.get(&rule[1..=rule.len()]),
            _ => None,
        }
    }
    #[inline]
    pub fn get_mut(&mut self, rule: &str) -> Option<&mut Rule> {
        self.rule_map.get_mut(rule)
    }
    #[inline]
    pub fn get_inline_mut(&mut self, rule: &str) -> Option<&mut Rule> {
        match self.rule_map.get(rule) {
            Some(_) => self.rule_map.get_mut(rule),
            // Check manual inlining
            None if rule.starts_with("_") => self.rule_map.get_mut(&rule[1..=rule.len()]),
            _ => None,
        }
    }
    #[inline]
    pub fn insert(&mut self, name: String, rule: Rule) -> Option<Rule> {
        self.rule_map.insert(name, rule)
    }
    #[inline]
    pub fn keys(&self) -> Keys<String, Rule> {
        self.rule_map.keys()
    }
    #[inline]
    pub fn rules(&self) -> Values<String, Rule> {
        self.rule_map.values()
    }
    #[inline]
    pub fn named_rules(&self) -> Vec<Rule> {
        self.rule_map.values().cloned().filter(|r| !r.auto_inline).collect()
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
