use super::*;
use character_set::builtin::{general_category, property_bool, property_values::PROPERTY_VALUES, script_extension};
use std::collections::BTreeMap;
use yggdrasil_error::YggdrasilError;

const GENERAL_CATEGORY: usize = 36;

impl ExpressionKind {
    pub fn rule(name: &str) -> Self {
        let data = match name {
            "ANY" => DataKind::CharacterAny,
            "XID_START" | "XID_CONTINUE" => DataKind::CharacterBuiltin(name.to_string()),
            _ => return Self::Rule(Box::new(RuleReference::new(name))),
        };
        Self::Data(Box::new(data))
    }
    pub fn regex_category(name: &str) -> Result<String, YggdrasilError> {
        let key = RegexCategory::normalize_name(name);
        if RegexCategory::default().inner.contains_key(&key) {
            Ok(key)
        }
        else {
            Err(YggdrasilError::language_error(format!("Unknown RegexCategory {}", name)))
        }
    }

    pub fn builtin(_name: &str) -> Option<Self> {
        todo!();
        // let ranges = BUILTIN_CHARACTER_RANGES.get(name)?;
        // let set = CharacterSet::from_ranges(ranges);
        // Some(Self::Data(Box::new(DataKind::CharacterSet(set))))
    }
}

type CategoryRange = BTreeMap<String, &'static [(char, char)]>;

pub struct RegexCategory {
    inner: CategoryRange,
}

impl Debug for RegexCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut w = &mut f.debug_struct("RegexCategory");
        for (key, value) in &self.inner {
            let ptr = (*value).as_ptr();
            w = w.field(key, &ptr)
        }
        w.finish()
    }
}

impl Default for RegexCategory {
    fn default() -> Self {
        Self { inner: Self::normalize(RegexCategory::load_data()) }
    }
}

impl RegexCategory {
    fn general_map(&self) -> BTreeMap<String, String> {
        let general = PROPERTY_VALUES[GENERAL_CATEGORY].1;
        BTreeMap::from_iter(general.iter().map(|(k, v)| (k.to_string(), v.to_string())))
    }
    fn load_data() -> CategoryRange {
        let mut out = BTreeMap::default();
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
        for (group, range) in property_bool::BY_NAME {
            out.insert(group.to_string(), *range);
        }
        for (group, range) in script_extension::BY_NAME {
            out.insert(group.to_string(), *range);
        }
        out
    }

    fn normalize(map: CategoryRange) -> CategoryRange {
        map.into_iter().map(|(k, v)| (Self::normalize_name(&k), v)).collect()
    }

    fn normalize_name(name: &str) -> String {
        name.chars().filter(|c| *c != ' ' && *c != '_').map(|c| c.to_ascii_lowercase()).collect()
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
