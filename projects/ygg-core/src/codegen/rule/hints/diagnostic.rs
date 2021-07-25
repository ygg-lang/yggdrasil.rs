use crate::codegen::rule::from_ast::FilePosition;
use super::*;

pub fn top_area_error(src: &str, msg: &str, range: (usize, usize), file: &FilePosition) -> Diagnostic {
    Diagnostic {
        range: file.get_lsp_range(range),
        severity: Some(DiagnosticSeverity::Warning),
        code: None,
        code_description: None,
        source: Some(String::from(src)),
        message: String::from(msg),
        related_information: None,
        tags: None,
        data: None,
    }
}

#[rustfmt::skip]
pub fn duplicate_declaration_error(
    src: &str,
    msg: impl Into<String>,
    this: (usize, usize),
    last: (usize, usize),
    file: &FilePosition
) -> Diagnostic {
    let info = DiagnosticRelatedInformation { location: Location { uri: file.url.to_owned(), range: file.get_lsp_range(last) }, message: msg.into() };
    Diagnostic {
        range: file.get_lsp_range(this),
        severity: Some(DiagnosticSeverity::Error),
        code: None,
        code_description: None,
        source: Some(String::from(src)),
        message: String::from("Duplicate declaration detected"),
        related_information: Some(vec![info]),
        tags: None,
        data: None,
    }
}
