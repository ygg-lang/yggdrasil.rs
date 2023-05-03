use super::*;
use crate::{
    grammar::GrammarInfo,
    nodes::{ChoiceExpression, ConcatExpression},
};

pub mod counter;

#[derive(Debug)]
pub enum FieldKind {
    Rule(String),
    IgnoreText,
    IgnoreRegex,
    IgnoreComment,
    IgnoreWhitespace,
}

impl Display for FieldKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
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
    pub fn field_name(&self) -> String {
        self.bind.to_case(Case::Snake)
    }
    pub fn field_type(&self, grammar: &GrammarInfo) -> String {
        match &self.kind {
            FieldKind::Rule(r) => match grammar.rules.get(r) {
                Some(s) => s.node_name(),
                None => "".to_string(),
            },
            FieldKind::IgnoreText => "".to_string(),
            FieldKind::IgnoreRegex => "".to_string(),
            FieldKind::IgnoreComment => "".to_string(),
            FieldKind::IgnoreWhitespace => "".to_string(),
        }
    }
}

impl GrammarRule {
    pub fn class_fields(&self) -> YggdrasilVariants {
        match &self.body {
            GrammarBody::Class { term } => YggdrasilVariants { fields: term.field_map().fields },
            _ => unreachable!("do you filter with class?"),
        }
    }
    pub fn union_fields(&self) -> YggdrasilEnumerates {
        match &self.body {
            GrammarBody::Union { branches } => {
                let mut variants = BTreeMap::default();
                for expr in branches {
                    let field = expr.field_map();
                    match &expr.tag {
                        Some(s) => variants.insert(s.text.clone().to_case(Case::Pascal), field),
                        None => unreachable!("have you run remark?"),
                    };
                }
                YggdrasilEnumerates { variants }
            }
            _ => unreachable!("do you filter with `union`?"),
        }
    }
}

impl YggdrasilExpression {
    fn field_map(&self) -> YggdrasilVariants {
        // let tag = self.tag.as_ref().or(candidate);
        match &self.body {
            ExpressionBody::Choice(many) => many.field_map(),
            ExpressionBody::Concat(many) => many.field_map(),
            // a:x+
            // a:(x*)
            // a:(b:x)
            ExpressionBody::Unary(one) => one.field_map(self.tag.as_ref()),
            ExpressionBody::Rule(one) => match &self.tag {
                Some(s) => YggdrasilVariants::rule(s, one, FieldCounter::ONE),
                None => YggdrasilVariants::default(),
            },
            _ => YggdrasilVariants::default(),
        }
    }
}

impl UnaryExpression {
    fn field_map(&self, candidate: Option<&YggdrasilIdentifier>) -> YggdrasilVariants {
        let count = self.counter();
        let tag = self.base.tag.as_ref().or(candidate);
        match &self.base.body {
            ExpressionBody::Concat(v) => v.field_map(),
            ExpressionBody::Choice(v) => v.field_map(),
            // base:Rule*         => tag = base
            // outer:(Rule)*      => tag = outer
            // outer:(base:Rule)* => tag = base
            ExpressionBody::Unary(v) => {
                let mut base = v.field_map(tag);
                base *= count;
                base
            }
            ExpressionBody::Rule(r) => match tag {
                Some(s) => YggdrasilVariants::rule(s, r, count),
                None => YggdrasilVariants::default(),
            },
            _ => YggdrasilVariants::default(),
        }
    }
}

impl ConcatExpression {
    ///```ygg
    /// T? ~ T+ -> T*
    /// T? ~ T? -> T*
    /// ```
    fn field_map(&self) -> YggdrasilVariants {
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
    fn field_map(&self) -> YggdrasilVariants {
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
                    out *= item.counter();
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
            YggdrasilOperator::Positive => FieldCounter::ONE,
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
