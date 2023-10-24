use super::*;

// Class rule
#[derive(Default, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FieldMap {
    pub fields: BTreeMap<String, YggdrasilField>,
}

impl IntoIterator for FieldMap {
    type Item = YggdrasilField;
    type IntoIter = impl Iterator<Item = YggdrasilField>;

    fn into_iter(self) -> Self::IntoIter {
        self.fields.into_iter().map(|v| v.1)
    }
}

impl BitAndAssign for FieldMap {
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

impl BitOrAssign for FieldMap {
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

impl BitXorAssign for FieldMap {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (key, value) in rhs.fields {
            match self.fields.get_mut(&key) {
                Some(s) => {
                    s.count *= value.count;
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

impl FieldMap {
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

impl FieldMap {
    pub fn rule(bind: &YggdrasilIdentifier, rule: &RuleReference, counter: YggdrasilCounter) -> Self {
        let mut out = BTreeMap::default();
        let field = YggdrasilField {
            lhs: bind.text.clone(),
            rhs: rule.name.text.clone(),
            count: counter,
            boxing: rule.boxed,
            bind_position: vec![bind.span.get_range()],
            rule_position: vec![rule.name.span.get_range()],
        };
        out.insert(bind.text.clone(), field);
        Self { fields: out }
    }
}
