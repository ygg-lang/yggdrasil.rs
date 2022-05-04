// use lsp_types::DiagnosticSeverity;
//
// use crate::DiagnosticLevel;
//
// impl DiagnosticLevel {
//     /// Convert to lsp [`DiagnosticSeverity`]
//     pub fn into_severity(self) -> Option<DiagnosticSeverity> {
//         match self {
//             DiagnosticLevel::Custom(_) => None,
//             DiagnosticLevel::Info => Some(DiagnosticSeverity::HINT),
//             DiagnosticLevel::Warning => Some(DiagnosticSeverity::WARNING),
//             DiagnosticLevel::Error => Some(DiagnosticSeverity::ERROR),
//             DiagnosticLevel::Fatal => Some(DiagnosticSeverity::ERROR),
//         }
//     }
// }
