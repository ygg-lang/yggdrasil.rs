use super::*;

/// Add custom derive to the rule
///
/// # Examples
///
/// Derive `Clone`, `Debug`, `Eq`, `PartialEq`, `Hash` and `serde::{Serialize, Deserialize}`:
///
/// ```
/// use yggdrasil_ir::rule::RuleDerive;
/// let mut derive = RuleDerive::default();
/// derive.insert_derive("Clone", "");
/// derive.insert_derive("Debug", "");
/// derive.insert_derive("Eq", "");
/// derive.insert_derive("PartialEq", "");
/// derive.insert_derive("Hash", "");
/// derive.insert_derive("serde::Serialize", "serde");
/// derive.insert_derive("serde::Deserialize", "serde");
/// ```
///
/// Equivalent to:
///
/// ```ignore
/// #[derive(Clone, Debug, Eq, Hash, PartialEq)]
/// #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RuleDerive {
    derives: BTreeMap<String, BTreeSet<String>>,
}

impl Default for RuleDerive {
    fn default() -> Self {
        RuleDerive { derives: Default::default() }
    }
}

impl Display for RuleDerive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (feature, derives) in &self.derives {
            let derives = derives.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
            if feature.is_empty() {
                writeln!(f, "#[derive({derives})]")?
            }
            else {
                writeln!(f, "#[cfg_attr(feature = \"{}\", derive({derives}))]", feature)?
            }
        }
        Ok(())
    }
}

impl RuleDerive {
    /// Derive common derive traits
    pub fn builtin() -> Self {
        let mut derive = RuleDerive::default();
        derive.insert_derive("Clone", "");
        derive.insert_derive("Debug", "");
        derive.insert_derive("Eq", "");
        derive.insert_derive("PartialEq", "");
        derive.insert_derive("Hash", "");
        derive.insert_derive("serde::Serialize", "serde");
        derive.insert_derive("serde::Deserialize", "serde");
        derive
    }
    /// Insert new derive to the set
    pub fn insert_derive(&mut self, derive: &str, feature: &str) {
        match self.derives.get_mut(feature) {
            Some(s) => {
                s.insert(derive.to_string());
            }
            None => {
                let mut set = BTreeSet::default();
                set.insert(derive.to_string());
                self.derives.insert(feature.to_string(), set);
            }
        }
    }
}
