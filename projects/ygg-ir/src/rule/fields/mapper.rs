use super::*;

#[derive(Default)]
pub struct FieldMap {
    pub(crate) wrap: BTreeMap<String, YggdrasilField>,
}

impl FieldMap {
    pub fn rule(bind: &YggdrasilIdentifier, rule: &RuleReference, counter: FieldCounter) -> Self {
        let mut out = BTreeMap::default();
        let field = YggdrasilField {
            name: bind.text.clone(),
            kind: FieldKind::Rule(rule.name.text.clone()),
            count: counter,
            bind_position: vec![bind.range.clone()],
            rule_position: vec![rule.name.range.clone()],
        };
        out.insert(field.name.clone(), field);
        Self { wrap: out }
    }
}

impl BitAndAssign for FieldMap {
    fn bitand_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl BitOrAssign for FieldMap {
    fn bitor_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl BitXorAssign for FieldMap {
    fn bitxor_assign(&mut self, rhs: Self) {
        todo!()
    }
}
