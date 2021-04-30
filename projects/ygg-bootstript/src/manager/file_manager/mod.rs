pub use self::{file_store::FileStore, file_wrap::FileType, finger_print::FileFingerprint};
use crate::{
    codegen::{GrammarState, GrammarType},
    manager::HintItems,
    Result, YGGError,
};
use dashmap::{mapref::one::Ref, DashMap};
use lsp_types::Url;
use std::{fs, lazy::SyncLazy, path::Path};

use xxhash_rust::xxh3::xxh3_128;

mod file_store;
mod file_wrap;
mod finger_print;

pub static FILE_MANAGER: SyncLazy<FileManager> = SyncLazy::new(|| FileManager::default());

pub type ParseResult<T> = Result<(T, Option<HintItems>)>;

//#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct FileManager {
    store: DashMap<Url, FileStore>,
}

impl Default for FileManager {
    fn default() -> Self {
        Self { store: Default::default() }
    }
}

impl FileManager {
    pub fn load_cache(&self, _path: impl AsRef<Path>) -> Result<()> {
        unimplemented!()
    }
    pub fn save_cache(&self, _path: impl AsRef<Path>) -> Result<()> {
        unimplemented!()
    }

    pub fn update_url(&self, url: Url) -> Result<()> {
        let new = FileFingerprint::new(&url)?;
        match self.store.get(&url) {
            Some(old) if old.eq(&new) => return Ok(()),
            _ => (),
        }
        let file = FileStore::load_url(&url, new)?;
        self.store.insert(url, file);
        Ok(())
    }
    #[inline]
    pub fn get_file(&self, url: &Url) -> Option<Ref<'_, Url, FileStore>> {
        self.store.get(&url)
    }
    pub fn get_symbol(&self, url: &Url, _symbol: &str) -> Result<()> {
        let _f = self.get_file(url);
        unimplemented!()
    }
    pub fn get_type(&self, _ty: &str) -> Result<()> {
        unimplemented!()
    }
}

impl FileManager {
    pub async fn parse_file(&self, url: &Url) -> ParseResult<Ref<'_, Url, FileStore>> {
        let hints;
        match url.to_file_path()?.extension().and_then(|e| e.to_str()) {
            Some("toml") => {
                hints = self.parse_type(url).await?.1;
                match self.get_file(url) {
                    Some(s) => Ok((s, hints)),
                    None => Err(YGGError::Unreachable),
                }
            }
            Some("ygg") | Some("yg") => {
                hints = self.parse_grammar(url).await?.1;
                match self.get_file(url) {
                    Some(s) => Ok((s, hints)),
                    None => Err(YGGError::Unreachable),
                }
            }
            _ => Err(YGGError::language_error("Unsupported file extension")),
        }
    }

    pub async fn parse_type(&self, _url: &Url) -> ParseResult<&GrammarType> {
        unimplemented!()
    }

    pub async fn parse_grammar(&self, url: &Url) -> ParseResult<GrammarState> {
        self.update_url(url.to_owned())?;
        self.store
            .get_mut(url)
            .ok_or(YGGError::language_error("Grammar not found"))?
            .value_mut()
            .parse_ygg(url.to_owned())
            .await
    }
}
