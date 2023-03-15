use super::*;

#[derive(Copy, Clone, Debug)]
pub struct FieldCounter {
    min: u16,
    max: u16,
}

pub enum FieldCounterType {
    Never,
    Optional,
    One,
    Array,
    ArrayNonZero,
}

impl FieldCounter {
    pub const NEVER: Self = Self { min: 0, max: 0 };
    pub const OPTIONAL: Self = Self { min: 0, max: 1 };
    pub const ONE: Self = Self { min: 1, max: 1 };
    pub const MANY: Self = Self { min: 0, max: u16::MAX };
    pub const MANY1: Self = Self { min: 1, max: u16::MAX };
}

impl FieldCounter {
    pub fn is_never(&self) -> bool {
        self.min == 0 && self.max == 0
    }
    pub fn as_count(&self) -> FieldCounterType {
        if self.is_never() {
            return FieldCounterType::Never;
        }
        if self.max == 1 {
            if self.min.is_zero() { FieldCounterType::Optional } else { FieldCounterType::One }
        }
        else {
            if self.min.is_zero() { FieldCounterType::Array } else { FieldCounterType::ArrayNonZero }
        }
    }
}

impl FieldCounterType {
    pub fn rust_container(&self, rule: &str) -> String {
        match self {
            FieldCounterType::Never => format!(""),
            FieldCounterType::Optional => format!("Option<{rule}>"),
            FieldCounterType::One => format!("{rule}"),
            FieldCounterType::Array => format!("Vec<{rule}>"),
            FieldCounterType::ArrayNonZero => format!("Vec<{rule}>"),
        }
    }
    pub fn cpp_container(&self, rule: &str) -> String {
        match self {
            FieldCounterType::Never => format!(""),
            FieldCounterType::Optional => format!("std::optional<{rule}>"),
            FieldCounterType::One => format!("{rule}"),
            FieldCounterType::Array => format!("std::vector<{rule}>"),
            FieldCounterType::ArrayNonZero => format!("std::vector<{rule}>"),
        }
    }
    pub fn csharp_container(&self, rule: &str) -> String {
        match self {
            FieldCounterType::Never => format!(""),
            FieldCounterType::Optional => format!("{rule}?"),
            FieldCounterType::One => format!("{rule}"),
            FieldCounterType::Array => format!("List<{rule}>"),
            FieldCounterType::ArrayNonZero => format!("List<{rule}>"),
        }
    }
    pub fn typescript_container(&self, rule: &str) -> String {
        self.csharp_container(rule)
    }
    pub fn kotlin_container(&self, rule: &str) -> String {
        match self {
            FieldCounterType::Never => format!(""),
            FieldCounterType::Optional => format!("{rule}?"),
            FieldCounterType::One => format!("{rule}"),
            FieldCounterType::Array => format!("Array<{rule}>"),
            FieldCounterType::ArrayNonZero => format!("Array<{rule}>"),
        }
    }
    pub fn python_container(&self, rule: &str) -> String {
        match self {
            FieldCounterType::Never => format!(""),
            FieldCounterType::Optional => format!("Optional[{rule}]"),
            FieldCounterType::One => format!("{rule}"),
            FieldCounterType::Array => format!("List[{rule}]"),
            FieldCounterType::ArrayNonZero => format!("List[{rule}]"),
        }
    }
}

/// ```ygg
/// a? ~ a+ => a*
/// a? ~ a? => a*
/// ```
impl BitAndAssign for FieldCounter {
    fn bitand_assign(&mut self, rhs: Self) {
        self.min = self.min.saturating_add(rhs.min);
        self.max = self.max.saturating_add(rhs.max);
    }
}

/// ```ygg
/// a? | a+ => a*
/// a? | a? => a?
/// ```
impl BitOrAssign for FieldCounter {
    fn bitor_assign(&mut self, rhs: Self) {
        self.min = self.min.max(rhs.min);
        self.max = self.max.max(rhs.max);
    }
}

/// ```ygg
/// a?+ => a*
/// a?? => a?
/// ```
impl BitXorAssign for FieldCounter {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.min = self.min.max(rhs.min);
        self.max = self.max.max(rhs.max);
    }
}

impl BitXorAssign<FieldCounter> for FieldMap {
    fn bitxor_assign(&mut self, rhs: FieldCounter) {
        for x in self.wrap.values_mut() {
            x.count ^= rhs
        }
    }
}
