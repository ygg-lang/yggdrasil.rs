use super::*;
use std::collections::btree_map::IntoIter;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YggdrasilVariant {
    pub tag: Option<YggdrasilIdentifier>,
    pub branch: YggdrasilExpression,
}
