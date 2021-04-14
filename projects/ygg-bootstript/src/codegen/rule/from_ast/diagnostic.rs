use crate::codegen::convert_range;
use lsp_types::{Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, NumberOrString, Url};
use std::str::FromStr;
use tree_sitter::Range;

pub fn top_area_error(msg: &str, r: Range) -> Diagnostic {
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

pub fn duplicate_declaration_error(msg: &str, this: Range, url: &Option<Url>, last: Option<Range>) -> Diagnostic {
    let related_information = match url {
        Some(u) => Some(vec![DiagnosticRelatedInformation {
            location: Location { uri: u.to_owned(), range: convert_range(last.unwrap()) },
            message: String::from(msg),
        }]),
        None => None,
    };
    Diagnostic {
        range: convert_range(this),
        severity: Some(DiagnosticSeverity::Error),
        code: None,
        code_description: None,
        source: None,
        message: String::new(),
        related_information,
        tags: None,
        data: None,
    }
}
