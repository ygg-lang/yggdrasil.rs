use super::*;
use std::collections::btree_map::IntoIter;

pub struct YggdrasilEnumerates {
    pub variants: BTreeMap<String, YggdrasilVariants>,
}

impl IntoIterator for YggdrasilEnumerates {
    type Item = (String, YggdrasilVariants);
    type IntoIter = IntoIter<String, YggdrasilVariants>;

    fn into_iter(self) -> Self::IntoIter {
        self.variants.into_iter()
    }
}

impl YggdrasilEnumerates {}
