use super::*;
use std::fmt::Write;

impl DataKind {
    pub fn refine(&mut self) {
        match self {
            DataKind::Integer(_) => {}
            DataKind::String(s) => {
                if s.chars().count() == 1 {
                    unsafe { *self = DataKind::Character(s.chars().nth(0).unwrap_unchecked()) }
                }
            }
            DataKind::Rule(_) => {}
            DataKind::CharacterAny => {}
            DataKind::Character(_) => {}
            DataKind::CharacterRange(r) => {
                if r.start == r.end {
                    *self = DataKind::Character(r.start)
                }
            }
            DataKind::CharacterSet(set) => {
                let ranges = set.to_ranges();
                if ranges.len() == 1 {
                    unsafe { *self = DataKind::CharacterRange(ranges.get_unchecked(0).clone()) }
                }
            }
        }
    }
}

pub(super) fn string_display(s: &str, f: &mut Formatter<'_>) -> std::fmt::Result {
    if s.contains('\'') {
        return Ok(());
    }
    for char in s.chars() {
        match char {
            '\n' => f.write_str("\\n")?,
            _ => f.write_char(char),
        }
    }
    Ok(())
}
