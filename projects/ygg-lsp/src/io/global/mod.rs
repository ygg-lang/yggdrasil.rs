use state::Storage;
use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
};
use tokio::sync::RwLock;
use tower_lsp::lsp_types::{Url, *};

pub static FILE_STORAGE: Storage<RwLock<FileStateMap>> = Storage::new();

pub trait FileStateUpdate<T> {
    fn update(&mut self, p: T);
}

#[derive(Clone)]
pub struct FileStateMap {
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
        Self { inner: Default::default() }
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
        // do nothing
        let _ = p;
    }
}

impl FileStateUpdate<DidCloseTextDocumentParams> for FileStateMap {
    fn update(&mut self, p: DidCloseTextDocumentParams) {
        // do nothing
        let _ = p;
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
    pub fn read(&self, url: &Url) -> Option<String> {
        self.inner.get(url).map(|e| e.text.to_owned())
    }
}

pub fn initialize_global_storages() {
    FILE_STORAGE.set(RwLock::new(FileStateMap::default()));
}
