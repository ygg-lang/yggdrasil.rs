use super::*;

pub fn name_missing() -> CodeLens {
    CodeLens { range: Default::default(), command: None, data: Some(Value::String(String::from("Grammar name not found"))) }
}
