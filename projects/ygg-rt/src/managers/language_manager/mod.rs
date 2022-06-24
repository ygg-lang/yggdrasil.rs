// use std::collections::BTreeMap;
//

use crate::NodeID;
use std::collections::BTreeMap;

pub type LanguageID = usize;

pub struct LanguageManager {
    cache: BTreeMap<NodeID, LanguageID>,
}
