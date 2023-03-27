use super::*;

// Class rule
#[derive(Default)]
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

impl BitAndAssign for YggdrasilVariants {
    fn bitand_assign(&mut self, rhs: Self) {
        for (key, value) in rhs.fields {
            match self.fields.get_mut(&key) {
                Some(s) => {
                    s.count &= value.count;
                    s.bind_position.extend(value.bind_position);
                    s.rule_position.extend(value.rule_position);
                }
                None => {
                    self.fields.insert(key, value);
                }
            }
        }
    }
}

impl BitOrAssign for YggdrasilVariants {
    fn bitor_assign(&mut self, rhs: Self) {
        for (key, value) in rhs.fields {
            match self.fields.get_mut(&key) {
                Some(s) => {
                    s.count |= value.count;
                    s.bind_position.extend(value.bind_position);
                    s.rule_position.extend(value.rule_position);
                }
                None => {
                    self.fields.insert(key, value);
                }
            }
        }
    }
}

impl BitXorAssign for YggdrasilVariants {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (key, value) in rhs.fields {
            match self.fields.get_mut(&key) {
                Some(s) => {
                    s.count ^= value.count;
                    s.bind_position.extend(value.bind_position);
                    s.rule_position.extend(value.rule_position);
                }
                None => {
                    self.fields.insert(key, value);
                }
            }
        }
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

impl YggdrasilVariants {
    pub fn rule(bind: &YggdrasilIdentifier, rule: &RuleReference, counter: FieldCounter) -> Self {
        let mut out = BTreeMap::default();
        let field = YggdrasilField {
            bind: bind.text.clone(),
            kind: FieldKind::Rule(rule.name.text.clone()),
            count: counter,
            bind_position: vec![bind.range.clone()],
            rule_position: vec![rule.name.range.clone()],
        };
        out.insert(bind.text.clone(), field);
        Self { fields: out }
    }
}
