use super::*;
use crate::rule::FieldCounter;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum YggdrasilOperator {
    /// ```ygg
    /// !e
    /// ```
    Negative,
    /// e?
    Optional,
    /// e*
    Repeats,
    /// e+
    Repeat1,
    /// no such literal
    Boxing,
    /// e+
    RepeatsBetween(Option<u8>, Option<u8>),
    /// *e
    Recursive,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct UnaryExpression {
    pub base: YggdrasilExpression,
    pub operators: Vec<YggdrasilOperator>,
}

impl UnaryExpression {
    pub fn counter(&self) -> FieldCounter {
        s
    }
}

impl From<UnaryExpression> for ExpressionKind {
    fn from(e: UnaryExpression) -> Self {
        Self::Unary(Box::new(e))
    }
}

impl Add<YggdrasilOperator> for YggdrasilExpression {
    type Output = Self;

    fn add(self, o: YggdrasilOperator) -> Self::Output {
        match self.kind {
            ExpressionKind::Unary(node) => {
                let mut ops = node.operators;
                ops.push(o);
                let unary = UnaryExpression { base: node.base, operators: ops };
                YggdrasilExpression { kind: ExpressionKind::Unary(Box::new(unary)), remark: self.remark, tag: self.tag }
            }
            _ => {
                let unary = UnaryExpression { base: self, operators: vec![o] };
                ExpressionKind::Unary(Box::new(unary)).into()
            }
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
            YggdrasilOperator::Negative => {}
            YggdrasilOperator::Optional => {}
            YggdrasilOperator::Repeats => {}
            YggdrasilOperator::Repeat1 => {}
            YggdrasilOperator::Boxing => {}
            YggdrasilOperator::RepeatsBetween(_, _) => {}
            YggdrasilOperator::Recursive => {}
        }
    }
}
