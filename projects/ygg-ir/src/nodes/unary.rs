use super::*;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum YggdrasilOperator {
    /// ```ygg
    /// &e
    /// ```
    Positive,
    /// ```ygg
    /// !e
    /// ```
    Negative,
    /// no such literal
    Boxing,
    /// e+
    RepeatsBetween { min: u32, max: u32 },
    /// *e
    Recursive,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnaryExpression {
    pub base: Box<YggdrasilExpression>,
    pub operators: Vec<YggdrasilOperator>,
}

impl From<UnaryExpression> for YggdrasilExpression {
    fn from(e: UnaryExpression) -> Self {
        ExpressionBody::Unary(e).into()
    }
}

impl Add<YggdrasilOperator> for YggdrasilExpression {
    type Output = Self;

    fn add(self, o: YggdrasilOperator) -> Self::Output {
        match self.body {
            ExpressionBody::Unary(node) => {
                let mut ops = node.operators;
                ops.push(o);
                let unary = UnaryExpression { base: node.base, operators: ops };
                YggdrasilExpression { body: ExpressionBody::Unary(unary), remark: self.remark, tag: self.tag }
            }
            _ => {
                let unary = UnaryExpression { base: Box::new(self), operators: vec![o] };
                ExpressionBody::Unary(unary).into()
            }
        }
    }
}

impl YggdrasilOperator {
    pub const OPTIONAL: YggdrasilOperator = Self::RepeatsBetween { min: 0, max: 1 };
    pub const REPEATS: YggdrasilOperator = Self::RepeatsBetween { min: 0, max: u32::MAX };
    pub const REPEAT1: YggdrasilOperator = Self::RepeatsBetween { min: 1, max: u32::MAX };
}
