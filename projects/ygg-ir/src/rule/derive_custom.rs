use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

/// Add custom derive to the rule
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CustomDerive {
    /// Feature condition, empty if no feature needed
    pub feature: String,
    /// Custom derive names
    pub custom: BTreeSet<String>,
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

impl CustomDerive {
    /// Derive `serde::{Serialize, Deserialize}`
    pub fn serde() -> Self {
        Self::new(&["serde::Serialize", "serde::Deserialize"]).with_feature("serde")
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

#[test]
fn test_serde() {
    let custom = CustomDerive::serde();
    assert_eq!(custom.to_string(), "#[cfg_attr(serde, derive(serde::Serialize, serde::Deserialize))]");
    let custom = CustomDerive::new(&["serde::Serialize", "serde::Deserialize"]);
    assert_eq!(custom.to_string(), "#[derive(serde::Serialize, serde::Deserialize)]");
    let custom = CustomDerive::new(&["serde::Serialize", "serde::Deserialize"]).with_feature("serde");
    assert_eq!(custom.to_string(), "#[cfg_attr(serde, derive(serde::Serialize, serde::Deserialize))]");
}
