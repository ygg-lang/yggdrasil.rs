use super::*;

pub static COMPLETE_MACROS: SyncLazy<Vec<CompletionItem>> = SyncLazy::new(|| complete_macros());

pub fn complete_macros() -> Vec<CompletionItem> {
    let parsed = load_md_doc(include_str!("macros.md"));
    parsed.iter().map(|doc| build_command(&doc.cmd, &doc.short, &doc.long)).collect()
}

pub fn build_command(cmd: &str, short: &str, long: &str) -> CompletionItem {
    let doc = MarkupContent { kind: MarkupKind::Markdown, value: String::from(long) };
    CompletionItem {
        label: format!("@{}", cmd),
        kind: Some(CompletionItemKind::Function),
        detail: Some(String::from(short)),
        documentation: Some(Documentation::MarkupContent(doc)),
        deprecated: None,
        preselect: None,
        sort_text: Some(format!("0{}", cmd)),
        filter_text: None,
        insert_text: Some(format!("{}($0)", cmd)),
        insert_text_format: Some(InsertTextFormat::Snippet),
        insert_text_mode: None,
        text_edit: None,
        additional_text_edits: None,
        command: None,
        commit_characters: None,
        data: None,
        tags: None,
    }
}
