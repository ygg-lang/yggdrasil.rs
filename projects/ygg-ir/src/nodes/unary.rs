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
    pub base: Box<YggdrasilExpression>,
    pub operators: Vec<YggdrasilOperator>,
}

impl From<UnaryExpression> for ExpressionKind {
    fn from(e: UnaryExpression) -> Self {
        Self::Unary(e)
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
                YggdrasilExpression { kind: ExpressionKind::Unary(unary), remark: self.remark, tag: self.tag }
            }
            _ => {
                let unary = UnaryExpression { base: Box::new(self), operators: vec![o] };
                ExpressionKind::Unary(unary).into()
            }
        }
    }
}
