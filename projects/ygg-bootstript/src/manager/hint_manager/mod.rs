pub use self::item::HintItems;
use crate::{Result, YGGError, FILE_MANAGER};
use lsp_types::{CodeLens, Diagnostic, DocumentSymbol, Url};
use std::{
    collections::HashMap,
    lazy::SyncLazy,
    ops::{Add, AddAssign},
};
use tokio::sync::RwLock;

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
    pub async fn update(&mut self, url: &Url) -> Result<&HintItems> {
        if let Some(s) = FILE_MANAGER.write().await.parse_file(&url).await?.1 {
            self.items.insert(url.to_owned(), s);
        }
        self.get(url).ok_or(YGGError::Unreachable)
    }
}
