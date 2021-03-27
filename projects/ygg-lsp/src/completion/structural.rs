use super::*;

pub fn complete_table() -> Vec<CompletionItem> {
    (1..=9).flat_map(|i| (1..=9).map(move |j| build_table(i, j))).collect()
}

fn build_table(a: usize, b: usize) -> CompletionItem {
    // let doc = MarkupContent { kind: MarkupKind::Markdown, value: String::from(long) };
    let short = format!("build a {} row {} column table", a, b);

    let title = build_column((1..=b).map(|e| format!(" ${{{}:title{}}} ", e + 1, e)));
    let align = build_column((1..=b).map(|_| format!(" ------ ")));
    let empty = build_column((1..=b).map(|_| format!("        ")));
    let mut insert_text = vec![title, align];
    // insert_text.extend(vec![empty].repeat(a));
    insert_text.extend(vec![empty].into_iter().cycle().take(a));
    insert_text.extend_one(String::new());
    CompletionItem {
        label: format!("\u{200B}Table {} × {}", a, b),
        kind: Some(CompletionItemKind::Keyword),
        detail: Some(short),
        // documentation: Some(Documentation::MarkupContent(doc)),
        documentation: None,
        deprecated: None,
        preselect: None,
        sort_text: Some(format!("9{}{}", a, b)),
        filter_text: Some(format!("\\table{}x{}", a, b)),
        insert_text: Some(insert_text.join("\n")),
        insert_text_format: Some(InsertTextFormat::Snippet),
        text_edit: None,
        additional_text_edits: None,
        command: None,
        data: None,
        tags: None,
    }
}

fn build_column(item: impl Iterator<Item = String>) -> String {
    format!("|{}|", item.collect::<Vec<String>>().join("|"))
}
