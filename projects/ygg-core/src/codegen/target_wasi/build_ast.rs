use super::*;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};
use yggdrasil_ir::rule::GrammarBody;

impl<'i> RustWriteAST<'i> {
    pub fn token_variant(&self, rule: &GrammarRule) -> String {
        format!("{}Token::{}", self.grammar.name.text.to_upper_camel_case(), rule.name.text.to_upper_camel_case())
    }
    pub fn guest_trait(&self, rule: &GrammarRule) -> String {
        format!("Guest{}{}Node", self.grammar.name.text, rule.name.text).to_upper_camel_case()
    }
    pub fn union_variants(&self, rule: &GrammarRule) -> Vec<(String, String, String)> {
        match &rule.body {
            GrammarBody::Union { refined, .. } => refined
                .iter()
                .map(|(key, value)| {
                    (
                        key.to_snake_case(),
                        key.to_upper_camel_case(),
                        format!("{}{}Node", self.grammar.name.text.to_upper_camel_case(), value.to_upper_camel_case()),
                    )
                })
                .collect(),
            _ => panic!(),
        }
    }
    pub fn native_node(&self, rule: &GrammarRule) -> String {
        format!("{}NativeNode", rule.name.text.to_upper_camel_case())
    }
    pub fn host_name(&self) -> String {
        format!("{}Host", self.grammar.name.text.to_upper_camel_case())
    }
    pub fn parse_wasi(&self, rule: &GrammarRule) -> String {
        format!("parse_{}_{}", self.grammar.name.text.to_snake_case(), rule.name.text.to_snake_case())
    }
    pub fn wasi_node(&self, rule: &GrammarRule) -> String {
        format!("{}{}Node", self.grammar.name.text.to_upper_camel_case(), rule.name.text.to_upper_camel_case())
    }
    pub fn language_id(&self) -> String {
        self.grammar.name.text.to_snake_case()
    }
    pub fn language_name(&self) -> String {
        self.grammar.name.text.to_upper_camel_case()
    }
}

// 写一个 rust tree 的库, 要求支持创建节点, 左右添加节点, 插入子节点, 删除子节点等操作
// 基本结构如下所示
pub struct Node<T> {
    id: usize,
    arena: Arc<Mutex<Arena<T>>>,
}

pub struct Arena<T> {
    root: usize,
    nodes: Vec<ArenaData<T>>,
}

pub struct ArenaData<T> {
    parent: Option<usize>,
    first_sibling: usize,
    left_sibling: Option<usize>,
    right_sibling: Option<usize>,
    last_sibling: usize,
    first_children: Option<usize>,
    last_children: Option<usize>,
    data: T,
}
