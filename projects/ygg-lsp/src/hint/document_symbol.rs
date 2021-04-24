use super::*;

pub async fn document_symbol_provider(params: DocumentSymbolParams) -> Result<Option<DocumentSymbolResponse>> {
    let mut store = GRAMMAR_MANAGER.write().await;
    let s = match store.parse_grammar(&params.text_document.uri) {
        Ok(e) => Some(e.0.show_document_symbol()),
        Err(_) => None,
    };
    Ok(s)
}
