use crate::io::read_url;
use lspower::lsp::{Url, *};
use state::Storage;
use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
};
use tokio::sync::RwLock;
use yggdrasil_bootstript::{ast::YGGBuilder, codegen::GrammarState, GRAMMAR_MANAGER, GrammarManager, Result};


pub trait FileStateUpdate<T> {
    fn update(&mut self, p: T);
}

#[derive(Clone, Debug)]
pub struct FileState {
    version: usize,
    text: String,
}

impl Default for FileState {
    fn default() -> Self {
        Self { version: 0, text: String::new() }
    }
}

impl FileStateUpdate<DidOpenTextDocumentParams> for GrammarManager {
    fn update(&mut self, p: DidOpenTextDocumentParams) {
        let url = p.text_document.uri;
        let v = p.text_document.version as usize;
        let text = p.text_document.text;
        self.update_versioned(&url, v, text)
    }
}

impl FileStateUpdate<DidChangeTextDocumentParams> for GrammarManager {
    fn update(&mut self, p: DidChangeTextDocumentParams) {
        let url = p.text_document.uri;
        let v = p.text_document.version as usize;
        let text = p.content_changes.iter().rev().nth(0).map(|e| e.text.clone()).unwrap_or_default();
        self.update_versioned(&url, v, text)
    }
}

impl FileStateUpdate<DidSaveTextDocumentParams> for GrammarManager {
    fn update(&mut self, p: DidSaveTextDocumentParams) {
        match self.update_from_file(&p.text_document.uri) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}

impl FileStateUpdate<DidCloseTextDocumentParams> for GrammarManager {
    fn update(&mut self, p: DidCloseTextDocumentParams) {
        match self.update_from_file(&p.text_document.uri) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}



pub fn initialize_global_storages() {
    GRAMMAR_MANAGER.set(RwLock::new(GrammarManager::default()));
}
