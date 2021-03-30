mod code_lens;
mod document_symbol;
mod hover;

pub use self::{
    code_lens::code_lens_provider, document_symbol::document_symbol_provider, hover::hover_provider,
};
//use crate::io::read_url;
// use arc_rs::ParserConfig;
use serde_json::Value;
use tower_lsp::lsp_types::*;

pub fn code_action_provider(p: CodeActionParams) -> Option<CodeActionResponse> {
    let _ = p;
    let mut actions = vec![];
    actions.extend(extract_actions());
    return Some(actions);
}

fn extract_actions() -> Vec<CodeActionOrCommand> {
    vec![
        cmd("🗝 Extract key path as cite", "arc.extract.key", vec![Value::Bool(true)]),
        cmd("📮 Extract value as json", "arc.extract.value", vec![Value::Bool(true)]),
    ]
}

fn cmd(show: &str, run: &str, arg: Vec<Value>) -> CodeActionOrCommand {
    CodeActionOrCommand::Command(Command {
        title: String::from(show),
        command: String::from(run),
        arguments: Some(arg),
    })
}

#[allow(dead_code)]
fn action() -> CodeAction {
    CodeAction {
        title: "Save image to local".to_string(),
        kind: Some(CodeActionKind::SOURCE_ORGANIZE_IMPORTS),
        diagnostics: None,
        edit: None,
        command: None,
        is_preferred: Some(true),
        disabled: None,
        data: None,
    }
}
