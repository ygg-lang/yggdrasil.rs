use super::*;
use crate::nodes::UnaryExpression;

/// A rule parameter
#[derive(Clone, Debug)]
pub struct RuleParameter {
    /// Parameter kind
    pub kind: RuleParameterKind,
    /// Parameter name
    pub name: String,
    /// Parameter typing
    pub typing: String,
}

/// Parameter kind
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RuleParameterKind {
    /// A required parameter, equivalent to `T` in rust.
    Required,
    /// An optional parameter, equivalent to `Option<T>` in rust.
    Optional,
    /// A variadic parameter, equivalent to `Vec<T>` in rust.
    Variadic,
}

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
        assert_eq!(self.kind, GrammarRuleKind::Class);
        let mut parameters = vec![];

        return parameters;
    }

    pub fn collect_union_parameters(&self) -> BTreeMap<String, Vec<RuleParameter>> {
        assert_eq!(self.kind, GrammarRuleKind::Union);
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
                    let rule = RuleParameter { kind: RuleParameterKind::Required, name: "".to_string(), typing: rule.name.clone() };
                    out.insert(self.tag.clone(), rule);
                }
            }
            ExpressionKind::Choice(v) => {
                for branch in &v.branches {
                    for (k, v) in branch.collect_parameters() {
                        match out.get_mut(&k) {
                            Some(p) => {
                                // *p = p | v;
                                todo!()
                            }
                            None => {
                                out.insert(k, v);
                            }
                        }
                    }
                }
            }
            ExpressionKind::Concat(v) => {
                for branch in v.into_iter() {
                    for (k, v) in branch.collect_parameters() {
                        match out.get_mut(&k) {
                            Some(p) => {
                                // *p &= v;
                                todo!()
                            }
                            None => {
                                out.insert(k, v);
                            }
                        }
                    }
                }
            }
            ExpressionKind::Unary(v) => {
                todo!()
            }
            ExpressionKind::Data(_) => {}
        }
        return out;
    }
}

impl UnaryExpression {
    fn collect_operator_parameters(&self) -> Option<RuleParameterKind> {
        let mut inner = None;
        for o in &self.ops {
            match o {
                // `(a?)*`, `(a?)+`
                Operator::Repeats | Operator::Repeat1 => return Some(RuleParameterKind::Optional),
                Operator::Optional => inner = Some(RuleParameterKind::Optional),
                Operator::Boxing => {
                    unreachable!("unsupported now")
                }
                Operator::Recursive => {
                    unreachable!("unsupported now")
                }
                Operator::Remark => {
                    unreachable!("`^p` should be resolved before collecting parameters")
                }

                Operator::Negative => {}

                Operator::RepeatsBetween(_, _) => {}
            }
        }
        assert_ne!(inner, Some(RuleParameterKind::Required));
        inner
    }
}
