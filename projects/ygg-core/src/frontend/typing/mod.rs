use crate::frontend::RuleDerive;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug)]
pub struct GrammarType {}

impl RuleDerive {
    pub fn derived(&self) -> Vec<String> {
        let mut derived = vec![];
        if self.eq {
            derived.push("Eq".to_string())
        }
        derived
    }
}

impl Debug for RuleDerive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.derived().iter()).finish()
    }
}

impl Display for RuleDerive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
