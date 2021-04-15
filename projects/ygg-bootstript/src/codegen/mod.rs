mod rule;

pub use self::rule::GrammarState;

pub fn convert_range(r: tree_sitter::Range) -> lsp_types::Range {
    lsp_types::Range {
        start: lsp_types::Position { line: r.start_point.row as u32, character: r.start_point.column as u32 },
        end: lsp_types::Position { line: r.end_point.row as u32, character: r.end_point.column as u32 },
    }
}
