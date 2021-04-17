use crate::codegen::convert_range;
use lsp_types::{Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, NumberOrString, Range, Url};
use std::str::FromStr;


pub fn top_area_error(src: &str, msg: &str, range: Range) -> Diagnostic {
    Diagnostic {
        range,
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

pub fn duplicate_declaration_error(src:&str, msg: impl Into<String>, this: Range, url: &Option<Url>, last: Option<Range>) -> Diagnostic {
    let related_information = match url {
        Some(u) => Some(vec![DiagnosticRelatedInformation {
            location: Location { uri: u.to_owned(), range: last.unwrap() },
            message: msg.into(),
        }]),
        None => None,
    };
    Diagnostic {
        range: this,
        severity: Some(DiagnosticSeverity::Error),
        code: None,
        code_description: None,
        source: Some(String::from(src)),
        message: String::from("Duplicate declaration detected"),
        related_information,
        tags: None,
        data: None,
    }
}