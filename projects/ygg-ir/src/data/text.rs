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

impl YggdrasilText {
    pub fn new<S>(text: S, span: Range<usize>) -> Self
    where
        S: Display,
    {
        Self { text: text.to_string(), insensitive: false, range: span }
    }
    pub fn with_case(self, insensitive: bool) -> Self {
        Self { insensitive, ..self }
    }
}
