use convert_case::{Case, Casing};
use std::{borrow::Cow, fmt::Write};
use yggdrasil_ir::{grammar::GrammarInfo, rule::YggdrasilField};

pub fn safe_rust_id<'i, S>(raw: S) -> askama::Result<Cow<'i, str>>
where
    S: Into<Cow<'i, str>>,
{
    let text = raw.into();
    let keywords = &[
        "abstract", "alignof", "as", "become", "box", "break", "const", "continue", "crate", "do", "else", "enum", "extern",
        "false", "final", "fn", "for", "if", "impl", "in", "let", "loop", "macro", "match", "mod", "move", "mut", "offsetof",
        "override", "priv", "proc", "pub", "pure", "ref", "return", "Self", "self", "sizeof", "static", "struct", "super",
        "trait", "true", "type", "typeof", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
    ];
    if keywords.contains(&text.as_ref()) { Ok(Cow::Owned(format!("r#{text}"))) } else { Ok(text) }
}

pub fn field_type(field: &YggdrasilField, grammar: &GrammarInfo) -> askama::Result<String> {
    let mut w = String::new();
    match grammar.rules.get(&field.rhs) {
        Some(s) => writeln!(w, "pub {}: {},", safe_rust_id(&field.lhs)?, s.node_name())?,
        None => writeln!(w, "// Missing rule {}", field.lhs)?,
    }
    Ok(w)
}

pub fn snake_case<T: AsRef<str>>(text: T) -> askama::Result<String> {
    Ok(text.as_ref().to_case(Case::Snake))
}
