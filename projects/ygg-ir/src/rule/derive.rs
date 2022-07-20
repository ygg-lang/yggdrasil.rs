use super::*;
use crate::rule::derive_custom::CustomDerive;

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuleDerive {
    pub copy: bool,
    pub clone: bool,
    pub debug: bool,
    pub eq: bool,
    pub partial_eq: bool,
    pub hash: bool,
    pub serialize: bool,
    pub deserialize: bool,
    pub custom: Vec<CustomDerive>,
}

impl Default for RuleDerive {
    fn default() -> Self {
        todo!()
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
    pub fn new(name: &str, range: &Range<usize>, kind: GrammarRuleKind) -> Self {
        Self {
            name: name.to_string(),
            r#type: String::new(),
            document: String::new(),
            public: false,
            derives: RuleDerive::default(),
            auto_inline: false,
            auto_boxed: false,
            entry: false,
            kind,
            body: ExpressionNode::empty(),
            range: range.clone(),
        }
    }
}
