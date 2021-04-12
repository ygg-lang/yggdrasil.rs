use super::*;

pub static COMPLETE_MACROS: SyncLazy<Vec<CompletionItem>> = SyncLazy::new(|| complete_macros());

pub fn complete_macros() -> Vec<CompletionItem> {
    let mut out = vec![];
    out.extend(load_md_doc(include_str!("macros_args.md")).iter().map(|doc| build_command(doc, true)));
    out.extend(load_md_doc(include_str!("macros.md")).iter().map(|doc| build_command(doc, false)));
    return out;
}

pub fn build_command(doc: &DocumentString, args: bool) -> CompletionItem {
    let cmd = doc.cmd.to_owned();
    let short = doc.short.to_owned();
    let doc = MarkupContent { kind: MarkupKind::Markdown, value: doc.long.to_owned() };
    let insert_text = match args {
        true => {
            format!("{}($1)", cmd)
        }
        false => cmd.clone(),
    };
    CompletionItem {
        label: format!("{}", cmd),
        label_details: None,
        kind: Some(CompletionItemKind::Function),
        detail: Some(short),
        documentation: Some(Documentation::MarkupContent(doc)),
        deprecated: None,
        preselect: None,
        sort_text: None,
        filter_text: None,
        insert_text: Some(insert_text),
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
