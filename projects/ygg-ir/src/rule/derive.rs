use super::*;

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuleDerive {
    pub(crate) parser: Option<String>,
    pub(crate) debug: Option<String>,
    pub(crate) display: Option<String>,
    pub(crate) eq: bool,
    pub(crate) eq_partial: Option<String>,
    pub(crate) ord: bool,
    pub(crate) ord_partial: Option<String>,
    pub(crate) hash: Option<String>,
}

impl Default for RuleDerive {
    fn default() -> Self {
        Self { parser: None, debug: None, display: None, eq: false, eq_partial: None, ord: false, ord_partial: None, hash: None }
    }
}

impl RuleDerive {
    pub fn derived(&self) -> Vec<String> {
        let mut derived = vec![];
        if self.eq {
            derived.push("Eq".to_string())
        }
        derived
    }
}

impl Debug for RuleDerive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.derived().iter()).finish()
    }
}

impl Display for RuleDerive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#[derive({})]", self.derived().join(", "))
    }
}

impl GrammarRule {
    pub fn new(name: &str, range: &Range<usize>) -> Self {
        Self {
            name: name.to_string(),
            r#type: String::new(),
            document: String::new(),
            public: false,
            derives: RuleDerive::default(),
            auto_inline: false,
            auto_boxed: false,
            entry: false,
            kind: false,
            body: ExpressionNode::empty(),
            range: range.clone(),
        }
    }
}
