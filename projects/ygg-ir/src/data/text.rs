use super::*;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YggdrasilText {
    pub text: String,
    pub insensitive: bool,
    pub range: Range<usize>,
}

impl From<YggdrasilText> for YggdrasilExpression {
    fn from(value: YggdrasilText) -> Self {
        ExpressionBody::Text(value).into()
    }
}
