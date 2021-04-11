use tower_lsp::lsp_types::{Diagnostic, Url};
use crate::io::{FILE_STORAGE};

// use crate::io::read_url;

pub async fn diagnostics_provider(url: &Url) -> Vec<Diagnostic> {
    let mut store = FILE_STORAGE.get().write().await;
    let grammar = store.parse(url).unwrap();
    return grammar.show_diagnostic();
}
