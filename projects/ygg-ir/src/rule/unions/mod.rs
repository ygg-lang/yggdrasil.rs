use super::*;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YggdrasilVariant {
    pub tag: Option<YggdrasilIdentifier>,
    pub branch: YggdrasilExpression,
}
