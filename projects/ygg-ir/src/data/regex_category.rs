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

const GENERAL_CATEGORY: usize = 36;

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
