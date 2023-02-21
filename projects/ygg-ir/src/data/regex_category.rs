use super::*;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct RegularExpression {
    pub text: String,
    pub span: Range<usize>,
}

impl From<RegularExpression> for YggdrasilExpression {
    fn from(value: RegularExpression) -> Self {
        ExpressionKind::Regex(value).into()
    }
}
