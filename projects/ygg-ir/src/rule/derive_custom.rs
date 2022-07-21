use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

/// Add custom derive to the rule
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CustomDerive {
    /// Feature condition, empty if no feature needed
    pub feature: String,
    /// Custom derive names
    pub custom: BTreeSet<String>,
}

impl Display for CustomDerive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let derives = self.custom.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
        if self.feature.is_empty() {
            write!(f, "#[derive({derives})]",)
        }
        else {
            write!(f, r#"#[cfg_attr(feature = "{}", derive({derives}))]"#, self.feature,)
        }
    }
}

impl PartialOrd<Self> for CustomDerive {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CustomDerive {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.feature.cmp(&other.feature) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.custom.cmp(&other.custom),
        }
    }
}

impl Default for CustomDerive {
    fn default() -> Self {
        Self::new(&[])
    }
}

impl CustomDerive {
    /// Derive `serde::{Serialize, Deserialize}`
    pub fn serde() -> Self {
        Self::new(&["serde::Serialize", "serde::Deserialize"]).with_feature("serde")
    }
    pub fn builtin() -> Self {
        Self::new(&["Clone", "Debug", "Eq", "PartialEq", "Hash"])
    }
    /// Create new custom derive
    pub fn new(custom: &[&str]) -> Self {
        Self { feature: "".to_string(), custom: custom.iter().map(|s| s.to_string()).collect() }
    }
    /// Add feature gate to the derive
    pub fn with_feature(self, feature: &str) -> Self {
        Self { feature: feature.to_string(), ..self }
    }
    /// Add custom derive to the group
    pub fn with_custom(&mut self, custom: &str) {
        self.custom.insert(custom.to_string());
    }
}
