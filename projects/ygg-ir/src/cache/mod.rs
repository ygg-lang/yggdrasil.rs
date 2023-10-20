use crate::{grammar::GrammarInfo, rule::YggdrasilIdentifier};
use std::collections::BTreeMap;

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct CacheManager {
    grammars: BTreeMap<String, GrammarInfo>,
    external: ExternalManager,
}

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExternalManager {
    parser: BTreeMap<String, BTreeMap<ProgramLanguage, Vec<YggdrasilIdentifier>>>,
    inspector: BTreeMap<String, BTreeMap<ProgramLanguage, Vec<YggdrasilIdentifier>>>,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ProgramLanguage {
    Rust,
    Kotlin,
    Typescript,
}
