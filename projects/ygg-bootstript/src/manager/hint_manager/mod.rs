use crate::{ast::YGGBuilder, manager::file_manager::FileStore};
use lsp_types::{CodeLens, Diagnostic, Url};
use std::{collections::HashMap, lazy::SyncLazy};
use tokio::sync::RwLock;
use self::item::HintItems;

mod item;

pub static HINT_MANAGER: SyncLazy<RwLock<HintManager>> = SyncLazy::new(|| RwLock::new(HintManager::default()));

impl Default for HintManager {
    fn default() -> Self {
        Self { items: Default::default() }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HintManager {
    items: HashMap<Url, HintItems>,
}

impl HintManager {
    #[inline]
    pub fn get(&self, url: &Url) -> Option<&HintItems> {
        self.items.get(url)
    }
    #[inline]
    pub fn set(&mut self, url: Url, hint: HintItems) -> Option<HintItems> {
        self.items.insert(url, hint)
    }
}
