use tower_lsp::lsp_types::{Diagnostic, Url};

// use crate::io::read_url;

pub fn diagnostics_provider(url: &Url) -> Vec<Diagnostic> {
    let _ = url;

    return vec![
        // override event
        // Diagnostic {
        //     range: Range::new(Position::new(3, 0), Position::new(3, 100)),
        //     severity: Some(DiagnosticSeverity::Hint),
        //     code: None,
        //     source: Some("at line:column".to_string()),
        //     message: "This item had been override".to_string(),
        //     related_information: None,
        //     tags: None,
        // },
    ];
}
