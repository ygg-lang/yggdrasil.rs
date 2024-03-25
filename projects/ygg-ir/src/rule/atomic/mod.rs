use super::*;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GrammarAtomic {
    Optimized,
    Atomic,
    #[default]
    Combined,
}

impl GrammarAtomic {
    /// Mark the rule already optimized
    pub fn optimize(&mut self) {
        *self = GrammarAtomic::Optimized
    }
}
