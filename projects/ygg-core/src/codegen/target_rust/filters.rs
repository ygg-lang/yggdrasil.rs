use std::fmt::Write;
use yggdrasil_ir::{
    grammar::GrammarInfo,
    rule::{FieldKind, YggdrasilField},
};
pub fn safe_rust_id(raw: &str) -> askama::Result<String> {
    let keywords = &[
        "abstract", "alignof", "as", "become", "box", "break", "const", "continue", "crate", "do", "else", "enum", "extern",
        "false", "final", "fn", "for", "if", "impl", "in", "let", "loop", "macro", "match", "mod", "move", "mut", "offsetof",
        "override", "priv", "proc", "pub", "pure", "ref", "return", "Self", "self", "sizeof", "static", "struct", "super",
        "trait", "true", "type", "typeof", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
    ];
    if keywords.contains(&raw) { Ok(format!("r#{raw}")) } else { Ok(format!("{raw}")) }
}

pub fn field_type(field: &YggdrasilField, grammar: &GrammarInfo) -> askama::Result<String> {
    let mut w = String::new();
    match &field.kind {
        FieldKind::Rule(name) => match grammar.rules.get(name) {
            Some(s) => writeln!(w, "pub {}: {},", safe_rust_id(&field.bind)?, s.node_name())?,
            None => writeln!(w, "// Missing rule {}", field.bind)?,
        },
        FieldKind::IgnoreText => {}
        FieldKind::IgnoreRegex => {}
        FieldKind::IgnoreComment => {}
        FieldKind::IgnoreWhitespace => {}
    }
    Ok(w)
}
