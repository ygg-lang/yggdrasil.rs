use crate::io::read_url;
use state::Storage;
use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
};
use tokio::sync::RwLock;
use tower_lsp::lsp_types::{Url, *};
use yggdrasil_bootstript::{ast::YGGBuilder, codegen::GrammarManager, Result};

pub static FILE_STORAGE: Storage<RwLock<FileStateMap>> = Storage::new();

pub trait FileStateUpdate<T> {
    fn update(&mut self, p: T);
}

pub struct FileStateMap {
    builder: YGGBuilder,
    inner: HashMap<Url, FileState>,
}

#[derive(Clone, Debug)]
pub struct FileState {
    version: usize,
    text: String,
}

impl Debug for FileStateMap {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl Default for FileStateMap {
    fn default() -> Self {
        Self { builder: YGGBuilder::new().unwrap(), inner: Default::default() }
    }
}

impl Default for FileState {
    fn default() -> Self {
        Self { version: 0, text: String::new() }
    }
}

impl FileStateUpdate<DidOpenTextDocumentParams> for FileStateMap {
    fn update(&mut self, p: DidOpenTextDocumentParams) {
        let url = p.text_document.uri;
        let v = p.text_document.version as usize;
        let text = p.text_document.text;
        self.update_versioned(&url, v, text)
    }
}

impl FileStateUpdate<DidChangeTextDocumentParams> for FileStateMap {
    fn update(&mut self, p: DidChangeTextDocumentParams) {
        let url = p.text_document.uri;
        let v = p.text_document.version as usize;
        let text = p.content_changes.iter().rev().nth(0).map(|e| e.text.clone()).unwrap_or_default();
        self.update_versioned(&url, v, text)
    }
}

impl FileStateUpdate<DidSaveTextDocumentParams> for FileStateMap {
    fn update(&mut self, p: DidSaveTextDocumentParams) {
        match self.update_from_file(&p.text_document.uri) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}

impl FileStateUpdate<DidCloseTextDocumentParams> for FileStateMap {
    fn update(&mut self, p: DidCloseTextDocumentParams) {
        match self.update_from_file(&p.text_document.uri) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}

impl FileStateMap {
    fn update_versioned(&mut self, url: &Url, version: usize, content: String) {
        let new = FileState { version, text: content };
        if let Some(last) = self.inner.get(url) {
            if last.version >= new.version {
                return ();
            }
        }
        self.inner.insert(url.clone(), new);
    }
    fn update_from_file(&mut self, url: &Url) -> Result<String> {
        let content = read_url(url)?;
        let new = FileState { version: 0, text: content.to_owned() };
        self.inner.insert(url.clone(), new);
        Ok(content)
    }
    pub fn parse(&mut self, url: &Url) -> Result<GrammarManager> {
        match self.inner.get(url) {
            Some(s) => self.builder.update_by_text(&s.text)?,
            None => {
                let t = self.update_from_file(url)?;
                self.builder.update_by_text(&t)?
            }
        };
        Ok(self.builder.traverse()?.build_grammar(Some(url.to_owned()))?)
    }
}

pub fn initialize_global_storages() {
    FILE_STORAGE.set(RwLock::new(FileStateMap::default()));
}
