use super::*;

pub fn document_symbol_provider(args: DocumentSymbolParams) -> Option<DocumentSymbolResponse> {
    // TODO: read from cache
    // let cfg = ParserConfig::default();
    // let ast = match cfg.parse(&read_url(&args.text_document.uri)) {
    //     Ok(o) => o,
    //     Err(_) => return None,
    // };
    // let nested = match ast.toc(9).build_document().children {
    //     Some(v) => v,
    //     None => vec![],
    // };
    // Some(DocumentSymbolResponse::Nested(nested))
    return None;
}
