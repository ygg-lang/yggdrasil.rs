#![allow(dead_code, non_camel_case_types)]
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
