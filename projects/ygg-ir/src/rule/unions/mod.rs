use super::*;
use std::collections::btree_map::IntoIter;
#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YggdrasilEnumerate {
    pub variants: BTreeMap<String, FieldMap>,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YggdrasilVariant {
    pub tag: YggdrasilIdentifier,
    pub branch: YggdrasilExpression,
}

impl IntoIterator for YggdrasilEnumerate {
    type Item = (String, FieldMap);
    type IntoIter = IntoIter<String, FieldMap>;

    fn into_iter(self) -> Self::IntoIter {
        self.variants.into_iter()
    }
}

impl YggdrasilEnumerate {
    pub fn insert(&mut self, variant: &YggdrasilVariant) {
        let field = variant.branch.field_map();
        self.variants.insert(variant.tag.text.clone(), field);
    }
}
