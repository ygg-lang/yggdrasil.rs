#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GrammarInfo {
    /// File path of the grammar
    pub url: Option<Url>,
    pub name: Symbol,
    pub extensions: Vec<Symbol>,
    pub ignores: Vec<Symbol>,
    pub imports: BTreeMap<Url, Vec<SymbolAlias>>,
    pub exports: Vec<String>,
    pub rules: BTreeMap<String, GrammarRule>,
    pub macros: Vec<String>,
    pub rule_prefix: String,
    pub rule_suffix: String,
}

impl Default for GrammarInfo {
    fn default() -> Self {
        Self {
            url: None,
            is_grammar: false,
            name: Symbol { name: "".to_string(), range: Default::default() },
            extensions: vec![],
            ignores: vec![],
            imports: Default::default(),
            rules: Default::default(),
            rule_prefix: "".to_string(),
            rule_suffix: "Node".to_string(),
        }
    }
}
