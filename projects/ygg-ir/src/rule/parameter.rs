use std::collections::BTreeMap;

use super::*;

impl Display for RuleParameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            RuleParameterKind::Optional => {
                write!(f, "{}: Option<{}>", self.name, self.typing)
            }
            RuleParameterKind::Required => {
                write!(f, "{}: {}", self.name, self.typing)
            }
            RuleParameterKind::Variadic => {
                write!(f, "{}: Vec<{}>", self.name, self.typing)
            }
        }
    }
}

impl GrammarRule {
    pub fn collect_class_parameters(&self) -> Vec<RuleParameter> {
        assert!(!self.union);
        let mut parameters = vec![];
        return parameters;
    }

    pub fn collect_union_parameters(&self) -> BTreeMap<String, Vec<RuleParameter>> {
        assert!(self.union);
        let mut parameters = BTreeMap::new();
        return parameters;
    }
}
