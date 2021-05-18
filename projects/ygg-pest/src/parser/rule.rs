#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    EOI,
    program_entry_point,
    program,
    statement,
    empty_statement,
    eos,
    grammar_statement,
    grammar,
    import_statement,
    import,
    SYMBOL,
}
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MetaRule {
    pub rule: Rule,
    pub mark: Option<&'static str>,
}
