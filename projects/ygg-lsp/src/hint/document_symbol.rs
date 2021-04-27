use super::*;

#[rustfmt::skip]
pub async fn document_symbol_provider(params: DocumentSymbolParams) -> Result<Option<DocumentSymbolResponse>> {
    let url = params.text_document.uri;
    HINT_MANAGER.write().await.update(&url).await.ok();
    let out = match HINT_MANAGER.read().await.get(&url) {
        Some(s) => {Some(DocumentSymbolResponse::Nested(s.document_symbol.to_owned()))},
        None => {None}
    };
    Ok(out)
}
