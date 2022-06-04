use std::{
    collections::BTreeMap,
    ops::{BitAnd, BitOr},
};

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

// a? a
impl BitAnd<Self> for RuleParameterKind {
    type Output = Self;

    fn bitand(self, _: Self) -> Self::Output {
        RuleParameterKind::Variadic
    }
}

impl BitOr<Self> for RuleParameterKind {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        use RuleParameterKind::*;
        match (self, rhs) {
            (Optional, Optional) => Optional,
            (Variadic, _) | (_, Variadic) => Variadic,
            _ => Required,
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

impl ExpressionNode {
    pub fn collect_parameters(&self) -> BTreeMap<String, RuleParameter> {
        let mut out: BTreeMap<String, RuleParameter> = BTreeMap::new();
        match &self.kind {
            ExpressionKind::Function(_) => unreachable!("Macros should be expand before collecting parameters"),
            ExpressionKind::Rule(rule) => {
                if !self.tag.is_empty() {
                    out.insert(self.tag.clone(), RuleParameter::new(self.tag.clone(), rule.typing.clone(), RuleParameterKind::Required));
                }
            }
            ExpressionKind::Choice(v) => {
                for branch in &v.branches {
                    for (k, v) in branch.collect_parameters() {
                        match out.get_mut(&k) {
                            Some(p) => {
                                *p |= v;
                            }
                            None => {
                                out.insert(k, v);
                            }
                        }
                    }
                }
            }
            ExpressionKind::Concat(v) => {
                for branch in &v.into_iter() {
                    for (k, v) in branch.collect_parameters() {
                        match out.get_mut(&k) {
                            Some(p) => {
                                *p &= v;
                            }
                            None => {
                                out.insert(k, v);
                            }
                        }
                    }
                }
            }
            ExpressionKind::Unary(v) => v.base.collect_parameters(),
            ExpressionKind::Data(_) => {}
        }
        return out;
    }
}

impl UnaryExpression {
    pub fn collect_parameters(&self) -> BTreeMap<String, RuleParameter> {
        let mut out: BTreeMap<String, RuleParameter> = BTreeMap::new();
        match &self.ops {
            &_ => {}
        }
        return out;
    }
    fn collect_operator_parameters(&self) -> RuleParameter {
        let mut out: BTreeMap<String, RuleParameter> = BTreeMap::new();
        match &self.ops {
            &_ => {}
        }
        return out;
    }
}
