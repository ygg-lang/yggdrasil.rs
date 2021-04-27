use crate::codegen::convert_range;
use lsp_types::{CodeLens, Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, NumberOrString, Range, Url};
use serde_json::Value;
use std::str::FromStr;

pub use diagnostic::*;
pub use code_lens::*;

mod code_lens;
mod diagnostic;
