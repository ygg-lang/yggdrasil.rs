use super::*;

impl YggdrasilVariants {}

impl IntoIterator for YggdrasilVariants {
    type Item = YggdrasilField;
    type IntoIter = impl Iterator<Item = YggdrasilField>;

    fn into_iter(self) -> Self::IntoIter {
        self.fields.into_iter().map(|v| v.1)
    }
}
