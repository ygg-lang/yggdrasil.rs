use super::*;
use num::Zero;
use std::ops::AddAssign;

#[derive(Debug)]
pub struct FieldCounter {
    min: usize,
    max: usize,
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
    pub const MANY: Self = Self { min: 0, max: usize::MAX };
    pub const MANY1: Self = Self { min: 1, max: usize::MAX };
}

impl AddAssign for FieldCounter {
    fn add_assign(&mut self, rhs: Self) {
        self.min += rhs.min;
        self.max += rhs.max;
    }
}

impl FieldCounter {
    pub fn as_count(&self) -> FieldCounterType {
        if self.min == 0 && self.max == 0 {
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
    pub fn typescript_container(&self, rule: &str) -> String {
        self.csharp_container(rule)
    }
    pub fn kotlin_container(&self, rule: &str) -> String {
        if self.max == 1 {
            if self.min.is_zero() { format!("{rule}?") } else { format!("{rule}") }
        }
        else {
            if self.min.is_zero() { format!("Array<{rule}>") } else { format!("Array<{rule}>") }
        }
    }
    pub fn python_container(&self, rule: &str) -> String {
        if self.max == 1 {
            if self.min.is_zero() { format!("Optional[{rule}]") } else { format!("{rule}") }
        }
        else {
            if self.min.is_zero() { format!("List[{rule}]") } else { format!("List[{rule}]") }
        }
    }
    pub fn csharp_container(&self, rule: &str) -> String {
        if self.max == 1 {
            if self.min.is_zero() { format!("{rule}?") } else { format!("{rule}") }
        }
        else {
            if self.min.is_zero() { format!("List<{rule}>") } else { format!("List<{rule}>") }
        }
    }
}
