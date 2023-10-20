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
    RepeatsBetween(YggdrasilCounter),
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
