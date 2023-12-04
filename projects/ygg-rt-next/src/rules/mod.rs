use core::{fmt::Debug, hash::Hash};

pub trait YggdrasilRule: Clone + Debug + Eq + Hash + Ord + TryFrom<u32> {}

pub trait YggdrasilLanguage {
    type Rule: YggdrasilRule;
}
