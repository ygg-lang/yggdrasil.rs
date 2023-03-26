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
