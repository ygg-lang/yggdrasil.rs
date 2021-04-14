use std::str::FromStr;
use lsp_types::{DiagnosticSeverity, NumberOrString, Diagnostic};
use tree_sitter::Range;
use crate::codegen::convert_range;
use lsp_types::DiagnosticRelatedInformation;
use lsp_types::{Location, Url};

pub fn top_area_error(msg:&str, r: Range) -> Diagnostic {
    Diagnostic {
        range: convert_range(r),
        severity: Some(DiagnosticSeverity::Warning),
        code: None,
        code_description: None,
        source: None,
        message: String::from(msg),
        related_information: None,
        tags: None,
        data: None,
    }
}

pub fn duplicate_declaration_error(msg:&str, this: Range, url: &Option<Url>, last: Option<Range>) -> Diagnostic {
    let related_information = match url {
        Some(u) => {Some(vec![
            DiagnosticRelatedInformation {
                location: Location { uri: u.to_owned(), range: convert_range(last.unwrap()) },
                message: format!("Already declaration here!")
            }
        ])},
        None => None
    };
    Diagnostic {
        range: convert_range(this),
        severity: Some(DiagnosticSeverity::Error),
        code: None,
        code_description: None,
        source: None,
        message: String::from(msg),
        related_information,
        tags: None,
        data: None,
    }
}