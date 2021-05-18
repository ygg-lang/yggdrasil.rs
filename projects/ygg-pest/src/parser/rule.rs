#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    EOI,
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
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MetaRule {
    pub rule: Rule,
    pub mark: Option<String>
}