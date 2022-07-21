use super::*;
use crate::rule::derive_custom::CustomDerive;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuleDerive {
    pub derives: BTreeSet<CustomDerive>,
}

impl Default for RuleDerive {
    fn default() -> Self {
        let mut derives = BTreeSet::new();
        derives.insert(CustomDerive::builtin());
        derives.insert(CustomDerive::serde());
        Self { derives }
    }
}

impl Display for RuleDerive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for derive in &self.derives {
            writeln!(f, "{}", derive)?;
        }
        Ok(())
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

#[test]
fn test_rule_derive() {
    let mut derives = RuleDerive::default();
    println!("{}", derives);
}
