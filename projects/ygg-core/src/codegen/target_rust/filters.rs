pub fn safe_rust_id(raw: &str) -> askama::Result<String> {
    let keywords = &[
        "abstract", "alignof", "as", "become", "box", "break", "const", "continue", "crate", "do", "else", "enum", "extern",
        "false", "final", "fn", "for", "if", "impl", "in", "let", "loop", "macro", "match", "mod", "move", "mut", "offsetof",
        "override", "priv", "proc", "pub", "pure", "ref", "return", "Self", "self", "sizeof", "static", "struct", "super",
        "trait", "true", "type", "typeof", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
    ];
    if keywords.contains(&raw) { Ok(format!("r#{raw}")) } else { Ok(format!("{raw}")) }
}
