use super::*;
use crate::{
    grammar::GrammarInfo,
    nodes::{ChoiceExpression, ConcatExpression},
};

pub mod counter;
pub mod mapper;

pub enum FieldKind {
    Rule(String),
    IgnoreText,
    IgnoreRegex,
    IgnoreComment,
    IgnoreWhitespace,
}

/// ```ygg
/// name: Kind
/// ```
///
/// ```ignore
/// pub struct ANode {
///     a: T,
///     b: Option<T>,
///     c: Vec<T>,
///     span: Range<usize>
/// }
/// ```
pub struct YggdrasilField {
    pub bind: String,
    pub kind: FieldKind,
    pub count: FieldCounter,
    pub bind_position: Vec<Range<usize>>,
    pub rule_position: Vec<Range<usize>>,
}

impl YggdrasilField {
    pub fn is_valid(&self, grammar: &GrammarInfo) -> bool {
        grammar.rules.get(&self.bind).is_some()
    }
    pub fn field_name(&self) -> String {
        self.bind.to_case(Case::Snake)
    }
}

impl GrammarRule {
    pub fn class_fields(&self) -> YggdrasilVariants {
        assert_eq!(self.kind, GrammarRuleKind::Class, "do you filter with class?");
        let fields = self.body.as_ref().map(|s| s.field_map()).unwrap_or_default();
        YggdrasilVariants { fields: fields.wrap }
    }
}

impl YggdrasilExpression {
    fn field_map(&self) -> FieldMap {
        // let tag = self.tag.as_ref().or(candidate);
        match &self.kind {
            ExpressionKind::Choice(many) => many.field_map(),
            ExpressionKind::Concat(many) => many.field_map(),
            // a:x+
            // a:(x*)
            // a:(b:x)
            ExpressionKind::Unary(one) => one.field_map(self.tag.as_ref()),
            ExpressionKind::Rule(one) => match &self.tag {
                Some(s) => FieldMap::rule(s, one, FieldCounter::ONE),
                None => FieldMap::default(),
            },
            _ => FieldMap::default(),
        }
    }
}

impl UnaryExpression {
    fn field_map(&self, candidate: Option<&YggdrasilIdentifier>) -> FieldMap {
        let count = self.counter();
        let tag = self.base.tag.as_ref().or(candidate);
        match &self.base.kind {
            ExpressionKind::Concat(v) => v.field_map(),
            ExpressionKind::Choice(v) => v.field_map(),
            // base:Rule*         => tag = base
            // outer:(Rule)*      => tag = outer
            // outer:(base:Rule)* => tag = base
            ExpressionKind::Unary(v) => {
                let mut base = v.field_map(tag);
                base ^= count;
                base
            }
            ExpressionKind::Rule(r) => match tag {
                Some(s) => FieldMap::rule(s, r, count),
                None => FieldMap::default(),
            },
            _ => FieldMap::default(),
        }
    }
}

impl ConcatExpression {
    ///```ygg
    /// T? ~ T+ -> T*
    /// T? ~ T? -> T*
    /// ```
    fn field_map(&self) -> FieldMap {
        let (head, rest) = self.split();
        let mut map = head.field_map();
        for item in rest {
            map &= item.field_map();
        }
        map
    }
}

impl ChoiceExpression {
    ///```ygg
    /// T?+ -> *
    /// T?? -> ?
    /// ```
    fn field_map(&self) -> FieldMap {
        let (head, rest) = self.split();
        let mut map = head.field_map();
        for item in rest {
            map |= item.field_map();
        }
        map
    }
}

impl UnaryExpression {
    ///```ygg
    /// T?+ -> *
    /// T?? -> ?
    /// ```
    pub fn counter(&self) -> FieldCounter {
        match self.operators.split_first() {
            Some((head, rest)) => {
                let mut out = head.counter();
                for item in rest {
                    out ^= item.counter();
                }
                out
            }
            None => FieldCounter::ONE,
        }
    }
}

impl YggdrasilOperator {
    pub fn prefix(o: &str) -> YggdrasilOperator {
        match o {
            "*" => Self::Recursive,
            _ => unreachable!(),
        }
    }
    pub fn suffix(o: &str) -> YggdrasilOperator {
        match o {
            "?" => Self::Optional,
            "+" => Self::Repeats,
            "*" => Self::Repeat1,
            _ => unreachable!(),
        }
    }
    pub fn counter(&self) -> FieldCounter {
        match self {
            YggdrasilOperator::Negative => FieldCounter::NEVER,
            YggdrasilOperator::Optional => FieldCounter::OPTIONAL,
            YggdrasilOperator::Repeats => FieldCounter::MANY,
            YggdrasilOperator::Repeat1 => FieldCounter::MANY1,
            YggdrasilOperator::Boxing => FieldCounter::ONE,
            YggdrasilOperator::RepeatsBetween(_, _) => FieldCounter::MANY,
            YggdrasilOperator::Recursive => FieldCounter::ONE,
        }
    }
}
