use super::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YggdrasilCounter {
    pub min: u32,
    pub max: u32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FieldCounterType {
    Never,
    Optional,
    One,
    Array,
    ArrayNonZero,
}

impl Display for YggdrasilCounter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.max >= 65536 { write!(f, "[{}, ∞]", self.min) } else { write!(f, "[{}, {}]", self.min, self.max) }
    }
}

impl YggdrasilCounter {
    pub const NEVER: Self = Self { min: 0, max: 0 };
    pub const OPTIONAL: Self = Self { min: 0, max: 1 };
    pub const ONE: Self = Self { min: 1, max: 1 };
    pub const MANY: Self = Self { min: 0, max: u32::MAX };
    pub const MANY1: Self = Self { min: 1, max: u32::MAX };
}

impl YggdrasilCounter {
    pub fn new(min: u32, max: u32) -> Self {
        Self { min, max }
    }
    pub fn is_never(&self) -> bool {
        self.min == 0 && self.max == 0
    }
    pub fn is_one(&self) -> bool {
        self.min == 1 && self.max == 1
    }
    pub fn is_optional(&self) -> bool {
        self.min == 0 && self.max == 1
    }
    pub fn is_many(&self) -> bool {
        self.min == 0 && self.max > 1
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
    pub fn as_range(&self) -> Range<u32> {
        self.min..self.max
    }
}

/// ```ygg
/// a? ~ a+ => a*
/// a? ~ a? => a*
/// ```
impl BitAndAssign for YggdrasilCounter {
    fn bitand_assign(&mut self, rhs: Self) {
        self.min = self.min.saturating_add(rhs.min);
        self.max = self.max.saturating_add(rhs.max);
    }
}

/// ```ygg
/// a? | a+ => a*
/// a? | a? => a?
/// ```
impl BitOrAssign for YggdrasilCounter {
    fn bitor_assign(&mut self, rhs: Self) {
        self.min = self.min.max(rhs.min);
        self.max = self.max.max(rhs.max);
    }
}

/// ```ygg
/// a?+ => a*
/// a?? => a?
/// ```
impl MulAssign for YggdrasilCounter {
    fn mul_assign(&mut self, rhs: Self) {
        self.min = self.min.saturating_mul(rhs.min);
        self.max = self.max.saturating_mul(rhs.max);
    }
}

impl MulAssign<YggdrasilCounter> for FieldMap {
    fn mul_assign(&mut self, rhs: YggdrasilCounter) {
        for x in self.fields.values_mut() {
            x.count *= rhs
        }
    }
}
