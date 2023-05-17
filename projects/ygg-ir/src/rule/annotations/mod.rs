use super::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YggdrasilModifiers {
    pub identifiers: Vec<YggdrasilIdentifier>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YggdrasilMacroCall {
    pub name: YggdrasilNamepath,
    pub arguments: Vec<YggdrasilMacroArgument>,
    pub range: Range<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YggdrasilMacroArgument {
    pub key: Option<YggdrasilIdentifier>,
    pub value: YggdrasilExpression,
}

impl From<YggdrasilMacroCall> for YggdrasilExpression {
    fn from(value: YggdrasilMacroCall) -> Self {
        ExpressionBody::Call(value).into()
    }
}
