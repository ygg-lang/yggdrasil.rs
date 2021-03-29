use super::*;


pub fn hover_provider(p: HoverParams) -> Option<Hover> {
    let _ = p;
    // Some(Hover {
    //     contents: HoverContents::Markup(MarkupContent {
    //         kind: MarkupKind::Markdown,
    //         value: "![](https://projecteuler.net/images/icons/info.png)".to_string(),
    //     }),
    //     range: None,
    // })
    return None;
}