use lspower::{
    jsonrpc::{Error, Result},
    lsp::*,
};
use yggdrasil_core::manager::FileManager;

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
        let url = p.text_document.uri;
        p.content_changes
            .into_iter()
            .rev()
            .nth(0)
            .map(|e| e.text)
            .and_then(|text| self.update_url_text(url, text).ok());
        Ok(())
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
