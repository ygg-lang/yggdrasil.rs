use super::*;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct YggdrasilText {
    pub text: String,
    pub insensitive: bool,
    pub range: Range<usize>,
}

impl From<YggdrasilText> for YggdrasilExpression {
    fn from(value: YggdrasilText) -> Self {
        ExpressionKind::Text(value).into()
    }
}

impl From<RangeInclusive<char>> for YggdrasilExpression {
    fn from(value: RangeInclusive<char>) -> Self {
        ExpressionKind::CharacterRange(value).into()
    }
}
