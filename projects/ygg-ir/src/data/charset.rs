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
            DataKind::CharacterBuiltin(_) => {
                todo!()
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
            _ => f.write_char(char)?,
        }
    }
    Ok(())
}

pub(super) fn char_range_display(range: &Range<char>, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}-{}]", range.start, range.end)
}

pub(super) fn char_set_display(set: &CharacterSet, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[")?;
    for range in set.to_ranges() {
        write!(f, "{}-{}", range.start, range.end)?
    }
    write!(f, "]")
}