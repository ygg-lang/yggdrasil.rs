use std::collections::HashMap;
use std::fs;
use std::lazy::SyncLazy;
use std::sync::RwLock;
use lsp_types::Url;
use xxhash_rust::xxh3::xxh3_128;
use crate::{Result, YGGError};

pub static GRAMMAR_MANAGER: SyncLazy<RwLock<GrammarManager>> = SyncLazy::new(|| {
    RwLock::new(GrammarManager::default())
});



pub struct GrammarManager {
    file_store: HashMap<Url, FileStore>
}

impl Default for GrammarManager {
    fn default() -> Self {
        Self {
            file_store: Default::default()
        }
    }
}

impl GrammarManager {

}


pub struct FileStore {
    fingerprint: u128,
    data: FileType
}

impl FileStore {
    pub fn load_url(&self, url:&Url) -> Result<Self> {
        let path = url.to_file_path()?;
        let input = fs::read_to_string(path)?;
        let mut bytes = input.into_bytes();
        bytes.extend_from_slice(url.as_str().as_bytes());
        let data = match path.extension().and_then(|e|e.to_str()) {
            Some("toml") => {Self::parse_toml(input)},
            Some("ygg") | Some("yg") => {Self::parse_ygg(input)},
            _ => Err(YGGError::IOError {
                error: String::from()
            })
        }?;


        Ok(Self {
            fingerprint: xxh3_128(&bytes),
            data,
        })
    }
    pub fn parse_toml(_input: String)->Result<FileType> {
        unimplemented!()
    }
    pub fn parse_ygg(_input: String)->Result<FileType> {
        unimplemented!()
    }
}


pub enum FileType {
    Grammar(),
    Fragment(),
    TypeDefine(),
}