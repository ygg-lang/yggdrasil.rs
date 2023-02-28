use super::*;

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

pub(super) fn char_range_display(range: &RangeInclusive<char>, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}-{}]", range.start(), range.end())
}

pub(super) fn char_set_display(set: &CharacterSet, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[")?;
    for range in set.to_ranges() {
        write!(f, "{}-{}", range.start(), range.end())?
    }
    write!(f, "]")
}
