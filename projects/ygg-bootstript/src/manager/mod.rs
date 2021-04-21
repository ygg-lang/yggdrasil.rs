use self::file_store::{FileFingerprint, FileStore};
use crate::{Result, YGGError};
use lsp_types::Url;
use rkyv::{
    Archive, Deserialize, Serialize,
};
use std::{collections::HashMap, fs, lazy::SyncLazy, path::Path, rc::Rc, sync::RwLock};
use xxhash_rust::xxh3::xxh3_128;
use crate::ygg::ast::YGGBuilder;

mod file_store;

pub static GRAMMAR_MANAGER: SyncLazy<RwLock<GrammarManager>> = SyncLazy::new(|| {
    RwLock::new(GrammarManager::new().expect("Manager initialization failed"))
});

//#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct GrammarManager {
    builder: YGGBuilder,
    file_store: HashMap<Url, FileStore>,
}


impl GrammarManager {
    pub fn new()-> Result<Self> {
        Ok(Self {
            builder: YGGBuilder::new()?,
            file_store: Default::default()
        })
    }

    pub fn load_cache(&mut self, _path: impl AsRef<Path>) -> Result<()> {
        unimplemented!()
    }
    pub fn save_cache(&self, _path: impl AsRef<Path>) -> Result<()> {
        unimplemented!()
    }

    pub fn load_url(&mut self, url: Url) -> Result<()> {
        let new = FileFingerprint::new(&url)?;
        match self.file_store.get(&url) {
            Some(old) if old.eq(&new) => return Ok(()),
            _ => (),
        }
        let file = FileStore::load_url(&url, new)?;
        self.file_store.insert(url, file);
        Ok(())
    }
    pub fn get_grammar(&self, url: &Url)-> Result<()> {
        let _f = self.file_store.get(&url);
        unimplemented!()
    }

    pub fn get_symbol(&self, url: &Url, _symbol: &str) -> Result<()> {
        let _f = self.file_store.get(&url);
        unimplemented!()
    }
    pub fn get_type(&self, _ty: &str) -> Result<()> {
        unimplemented!()
    }
}
