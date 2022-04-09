use super::*;
use character_set::builtin::property_values::PROPERTY_VALUES;
use std::collections::{HashMap, HashSet};

const GENERAL_CATEGORY: usize = 36;

impl ExpressionKind {
    pub fn rule(name: &str) -> Self {
        let data = match name {
            "ANY" => DataKind::CharacterAny,
            "XID_START" => {
                todo!();
                // DataKind::CharacterSet(name.to_string())
            }
            _ => return Self::Rule(Box::new(RuleReference::new(name))),
        };
        Self::Data(Box::new(data))
    }
    pub fn regex_category(name: &str) -> String {
        if let Some(s) = general_map().get(name) {
            return s.to_owned();
        }

        String::new()
    }

    pub fn builtin(name: &str) -> Option<Self> {
        todo!();
        // let ranges = BUILTIN_CHARACTER_RANGES.get(name)?;
        // let set = CharacterSet::from_ranges(ranges);
        // Some(Self::Data(Box::new(DataKind::CharacterSet(set))))
    }
}

pub struct BuiltinMap {
    inner: HashMap<String, &'static [(char, char)]>,
}

impl BuiltinMap {
    fn new() -> Self {
        let mut out = HashMap::default();
        for (short, long) in PROPERTY_VALUES[GENERAL_CATEGORY].1 {}
        Self { inner: out }
    }

    fn general_map(&self) -> HashMap<String, String> {
        let general = PROPERTY_VALUES[GENERAL_CATEGORY].1;
        HashMap::from_iter(general.iter().map(|(k, v)| (k.to_string(), v.to_string())))
    }
}

#[test]
fn find_general_category() {
    let mut id = 0;
    for (key, _) in PROPERTY_VALUES {
        if *key == "General_Category" {
            break;
        }
        id += 1;
    }
    assert_eq!(id, GENERAL_CATEGORY)
}
