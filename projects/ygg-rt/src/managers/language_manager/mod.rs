// use std::collections::BTreeMap;
//

use crate::NodeID;
use std::{
    collections::{hash_map::DefaultHasher, BTreeMap},
    hash::{Hash, Hasher},
};

pub type LanguageID = u64;

pub struct LanguageManager {
    cache: BTreeMap<NodeID, LanguageID>,
}

impl LanguageManager {
    pub fn id_from_name(name: &str) -> LanguageID {
        let mut hasher = DefaultHasher::new();
        name.to_lowercase().hash(&mut hasher);
        hasher.finish()
    }
}
