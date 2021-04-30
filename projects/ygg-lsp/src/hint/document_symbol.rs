use super::*;

#[rustfmt::skip]
pub async fn document_symbol_provider(p: DocumentSymbolParams) -> Result<Option<DocumentSymbolResponse>> {
    let url = p.text_document.uri;
    HINT_MANAGER.update(&url).await.ok();
    let out = match HINT_MANAGER.get(&url) {
        Some(s) => {Some(DocumentSymbolResponse::Nested(s.document_symbol.to_owned()))},
        None => {None}
    };
    Ok(out)
}
