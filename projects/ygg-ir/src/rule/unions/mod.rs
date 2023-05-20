use super::*;
use std::collections::btree_map::IntoIter;

pub struct YggdrasilEnumerates {
    pub variants: BTreeMap<String, FieldMap>,
}

impl IntoIterator for YggdrasilEnumerates {
    type Item = (String, FieldMap);
    type IntoIter = IntoIter<String, FieldMap>;

    fn into_iter(self) -> Self::IntoIter {
        self.variants.into_iter()
    }
}

impl YggdrasilEnumerates {}
