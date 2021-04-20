use crate::{Result, YGGError};
use lsp_types::Url;
use std::{collections::HashMap, fs, lazy::SyncLazy, sync::RwLock};
use xxhash_rust::xxh3::xxh3_128;
use self::file_store::{FileStore,FileFingerprint};

mod file_store;

pub static GRAMMAR_MANAGER: SyncLazy<RwLock<GrammarManager>> = SyncLazy::new(|| RwLock::new(GrammarManager::default()));

pub struct GrammarManager {
    file_store: HashMap<Url, FileStore>,
}

impl Default for GrammarManager {
    fn default() -> Self {
        Self { file_store: Default::default() }
    }
}

impl GrammarManager {
    pub fn load_url(&mut self, url: Url)->Result<()> {
        let new = FileFingerprint::new(&url)?;
        match self.file_store.get(&url){
            Some(old) if old.eq(&new) => {
                return Ok(())
            },
            _ => ()
        }
        let file = FileStore::load_url(&url, new)?;
        self.file_store.insert(url, file);
        Ok(())
    }
}
