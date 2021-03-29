use super::*;

pub static COMPLETE_CONSTANTS: SyncLazy<Vec<CompletionItem>> = SyncLazy::new(|| complete_constants());

pub fn complete_constants() -> Vec<CompletionItem> {
    let mut out = vec![];
    out.extend(
        load_md_doc(include_str!("constants_ascii.md"))
            .iter()
            .map(|doc| build_command(doc, CompletionItemKind::Constant)),
    );
    out.extend(
        load_md_doc(include_str!("constants_text.md"))
            .iter()
            .map(|doc| build_command(doc, CompletionItemKind::Text)),
    );
    out.extend(
        load_md_doc(include_str!("constants_op.md"))
            .iter()
            .map(|doc| build_command(doc, CompletionItemKind::Operator)),
    );
    return out;
}

pub fn build_command(doc: &DocumentString, kind: CompletionItemKind) -> CompletionItem {
    let cmd = doc.cmd.to_owned();
    let short = doc.short.to_owned();
    let doc = MarkupContent { kind: MarkupKind::Markdown, value: doc.long.to_owned() };
    CompletionItem {
        label: cmd.clone(),
        kind: Some(kind),
        detail: Some(short),
        documentation: Some(Documentation::MarkupContent(doc)),
        deprecated: None,
        preselect: None,
        sort_text: Some(format!("9{}",cmd.to_lowercase())),
        filter_text: None,
        insert_text: Some(cmd),
        insert_text_format: None,
        insert_text_mode: None,
        text_edit: None,
        additional_text_edits: None,
        command: None,
        commit_characters: None,
        data: None,
        tags: None,
    }
}
