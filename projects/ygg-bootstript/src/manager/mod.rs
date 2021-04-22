use std::{collections::HashMap, fs, lazy::SyncLazy, path::Path, rc::Rc, sync::RwLock};
use lsp_types::{Diagnostic, Url};
use rkyv::{
    Archive, Deserialize, Serialize,
};

use crate::{Result, YGGError};
use crate::codegen::GrammarState;
use crate::manager::file_store::FileType;
use crate::ygg::ast::YGGBuilder;
use self::file_store::{FileFingerprint, FileStore};

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

    pub fn update_url(&mut self, url: Url) -> Result<()> {
        let new = FileFingerprint::new(&url)?;
        match self.file_store.get(&url) {
            Some(old) if old.eq(&new) => return Ok(()),
            _ => (),
        }
        let file = FileStore::load_url(&url, new)?;
        self.file_store.insert(url, file);
        Ok(())
    }
    pub fn get_grammar(&self, url: &Url)-> Option<&GrammarState> {
        match &self.file_store.get(&url).map(|f| &f.data) {
            Some(FileType::Grammar(g)) => {
                Some(g)
            },
            _ => None
        }
    }

    pub fn get_symbol(&self, url: &Url, _symbol: &str) -> Result<()> {
        let _f = self.get_grammar(url);
        unimplemented!()
    }
    pub fn get_type(&self, _ty: &str) -> Result<()> {
        unimplemented!()
    }
}

impl GrammarManager {
    pub fn parse_grammar(&mut self, url: &Url) -> Result<(GrammarState, Vec<Diagnostic>)> {
        self.update_url(url.to_owned());
        let grammar = match self.get_grammar(url) {
            Some(s) => {Ok(s)},
            _ => {Err(YGGError::io_error("Grammar not found"))}
        }?;

        Ok(self.builder.traverse()?.build_grammar(Some(url.to_owned()))?)
    }
}
