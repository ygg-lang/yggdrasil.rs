use super::*;
use lsp_types::Command;

pub fn name_missing() -> CodeLens {
    let cmd = Command::new("Grammar name missing".to_string(), "ygg.insert_grammar".to_string(), None);

    CodeLens {
        range: Default::default(),
        command: Some(cmd),
        data: Some(Value::String(String::from("Grammar name not found"))),
    }
}
