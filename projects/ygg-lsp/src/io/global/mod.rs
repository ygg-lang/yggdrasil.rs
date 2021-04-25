use lspower::{
    jsonrpc::{Error, Result},
    lsp::*,
};
use yggdrasil_bootstript::FileManager;

pub trait FileStateUpdate<T> {
    fn update(&mut self, p: T) -> Result<()>;
}

impl FileStateUpdate<DidOpenTextDocumentParams> for FileManager {
    fn update(&mut self, p: DidOpenTextDocumentParams) -> Result<()> {
        let url = p.text_document.uri;
        self.update_url(url).map_err(|_| Error::internal_error())
    }
}

impl FileStateUpdate<DidChangeTextDocumentParams> for FileManager {
    fn update(&mut self, p: DidChangeTextDocumentParams) -> Result<()> {
        // TODO: Incremental update
        // let url = p.text_document.uri;
        // let v = p.text_document.version as usize;
        // let text = p.content_changes.iter().rev().nth(0).map(|e| e.text.clone()).unwrap_or_default();
        // self.update_versioned(&url, v, text)
        let url = p.text_document.uri;
        self.update_url(url).map_err(|_| Error::internal_error())
    }
}

impl FileStateUpdate<DidSaveTextDocumentParams> for FileManager {
    fn update(&mut self, p: DidSaveTextDocumentParams) -> Result<()> {
        let url = p.text_document.uri;
        self.update_url(url).map_err(|_| Error::internal_error())
    }
}

impl FileStateUpdate<DidCloseTextDocumentParams> for FileManager {
    fn update(&mut self, p: DidCloseTextDocumentParams) -> Result<()> {
        let url = p.text_document.uri;
        self.update_url(url).map_err(|_| Error::internal_error())
    }
}
