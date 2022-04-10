use super::*;
use character_set::builtin::{general_category, property_values::PROPERTY_VALUES};
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
        String::new()
    }

    pub fn builtin(name: &str) -> Option<Self> {
        todo!();
        // let ranges = BUILTIN_CHARACTER_RANGES.get(name)?;
        // let set = CharacterSet::from_ranges(ranges);
        // Some(Self::Data(Box::new(DataKind::CharacterSet(set))))
    }
}

pub struct RegexCategory {
    inner: HashMap<String, &'static [(char, char)]>,
}

impl Debug for RegexCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // *const
        let mut w = &mut f.debug_struct("RegexCategory");

        f.debug_map().entries(self.inner.iter()).finish()
    }
}

impl Default for RegexCategory {
    fn default() -> Self {
        let mut out = HashMap::default();
        for (group, range) in general_category::BY_NAME {
            out.insert(group.to_string(), *range);
        }
        for (short, long) in PROPERTY_VALUES[GENERAL_CATEGORY].1 {
            let range = match out.get(*long) {
                Some(s) => *s,
                None => continue,
            };
            out.insert(short.to_string(), range);
        }
        Self { inner: out }
    }
}

impl RegexCategory {
    fn general_map(&self) -> HashMap<String, String> {
        let general = PROPERTY_VALUES[GENERAL_CATEGORY].1;
        HashMap::from_iter(general.iter().map(|(k, v)| (k.to_string(), v.to_string())))
    }
}

#[test]
fn test() {
    println!("{:#?}", RegexCategory::default())
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
