use super::*;

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GrammarAtomic {
    Optimized,
    Atomic,
    Combined,
}

impl GrammarAtomic {
    /// Mark the rule already optimized
    pub fn optimize(&mut self) {
        *self = GrammarAtomic::Optimized
    }
}
