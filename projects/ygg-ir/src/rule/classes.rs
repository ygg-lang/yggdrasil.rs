use super::*;

// Class rule
pub struct YggdrasilVariants {
    pub fields: BTreeMap<String, YggdrasilField>,
}

impl IntoIterator for YggdrasilVariants {
    type Item = YggdrasilField;
    type IntoIter = impl Iterator<Item = YggdrasilField>;

    fn into_iter(self) -> Self::IntoIter {
        self.fields.into_iter().map(|v| v.1)
    }
}

impl YggdrasilVariants {
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }
    pub fn as_single(&self) -> Option<&YggdrasilField> {
        if self.fields.len() != 1 {
            None
        }
        else {
            let pair = self.fields.first_key_value()?;
            return Some(pair.1);
        }
    }
}
