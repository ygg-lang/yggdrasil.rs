use super::*;
use std::collections::btree_map::IntoIter;
use yggdrasil_parser::bootstrap::IdentifierNode;

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YggdrasilEnumerate {
    pub variants: BTreeMap<String, FieldMap>,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YggdrasilVariant {
    pub tag: Option<YggdrasilIdentifier>,
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
        match &variant.tag {
            Some(s) => {
                self.variants.insert(s.text.clone(), field);
            }
            None => unreachable!("have you run remark?"),
        };
    }
}

impl YggdrasilVariant {
    pub fn remark(&mut self, tag: YggdrasilIdentifier) {
        self.tag = Some(tag)
    }
    pub fn unmark(&mut self) {
        match &self.tag {
            Some(s) => self.branch.tag = Some(s.clone()),
            None => unreachable!("have you run remark?"),
        }
    }
}
