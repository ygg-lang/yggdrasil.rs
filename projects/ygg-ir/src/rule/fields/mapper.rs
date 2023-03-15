use super::*;

#[derive(Default)]
pub struct FieldMap {
    pub(crate) wrap: BTreeMap<String, YggdrasilField>,
}

impl FieldMap {
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
        Self { wrap: out }
    }
}

impl BitAndAssign for FieldMap {
    fn bitand_assign(&mut self, rhs: Self) {
        for (key, value) in rhs.wrap {
            match self.wrap.get_mut(&key) {
                Some(s) => {
                    s.count &= value.count;
                    s.bind_position.extend(value.bind_position);
                    s.rule_position.extend(value.rule_position);
                }
                None => {
                    self.wrap.insert(key, value);
                }
            }
        }
    }
}

impl BitOrAssign for FieldMap {
    fn bitor_assign(&mut self, rhs: Self) {
        for (key, value) in rhs.wrap {
            match self.wrap.get_mut(&key) {
                Some(s) => {
                    s.count |= value.count;
                    s.bind_position.extend(value.bind_position);
                    s.rule_position.extend(value.rule_position);
                }
                None => {
                    self.wrap.insert(key, value);
                }
            }
        }
    }
}

impl BitXorAssign for FieldMap {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (key, value) in rhs.wrap {
            match self.wrap.get_mut(&key) {
                Some(s) => {
                    s.count ^= value.count;
                    s.bind_position.extend(value.bind_position);
                    s.rule_position.extend(value.rule_position);
                }
                None => {
                    self.wrap.insert(key, value);
                }
            }
        }
    }
}
