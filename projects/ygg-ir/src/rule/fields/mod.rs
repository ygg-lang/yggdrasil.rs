use super::*;
use crate::{
    grammar::GrammarInfo,
    nodes::{ChoiceExpression, ConcatExpression},
};
use std::iter::from_generator;

pub mod counter;
pub mod mapper;

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
        assert_eq!(self.kind, GrammarRuleKind::Class, "do you filter with class?");
        let fields = self.body.as_ref().map(|s| s.field_map()).unwrap_or_default();
        YggdrasilVariants { fields: fields.wrap }
    }
    pub fn union_fields(&self) -> YggdrasilEnumerates {
        assert_eq!(self.kind, GrammarRuleKind::Union, "do you filter with union?");
        let mut variants = BTreeMap::default();
        for expr in self.get_branches() {
            let field = YggdrasilVariants { fields: expr.field_map().wrap };
            match &expr.tag {
                Some(s) => variants.insert(s.text.clone().to_case(Case::Pascal), field),
                None => {
                    variants.insert("buggggggggggggggg".to_string(), field)
                    // unreachable!("have you run remark?")
                }
            };
        }
        YggdrasilEnumerates { variants }
    }
    fn get_branches<'i>(&'i self) -> impl Iterator<Item = &'i YggdrasilExpression> + '_ {
        from_generator(move || match &self.body {
            Some(s) => match &s.body {
                ExpressionBody::Choice(e) => {
                    for item in &e.branches {
                        yield item
                    }
                }
                _ => yield s,
            },
            None => {}
        })
    }
}

impl YggdrasilExpression {
    fn field_map(&self) -> FieldMap {
        // let tag = self.tag.as_ref().or(candidate);
        match &self.body {
            ExpressionBody::Choice(many) => many.field_map(),
            ExpressionBody::Concat(many) => many.field_map(),
            // a:x+
            // a:(x*)
            // a:(b:x)
            ExpressionBody::Unary(one) => one.field_map(self.tag.as_ref()),
            ExpressionBody::Rule(one) => match &self.tag {
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
        match &self.base.body {
            ExpressionBody::Concat(v) => v.field_map(),
            ExpressionBody::Choice(v) => v.field_map(),
            // base:Rule*         => tag = base
            // outer:(Rule)*      => tag = outer
            // outer:(base:Rule)* => tag = base
            ExpressionBody::Unary(v) => {
                let mut base = v.field_map(tag);
                base ^= count;
                base
            }
            ExpressionBody::Rule(r) => match tag {
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
