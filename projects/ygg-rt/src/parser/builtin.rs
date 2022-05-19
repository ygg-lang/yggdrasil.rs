use super::*;

#[allow(clippy::needless_lifetimes)]
pub fn parse_char<'i>(state: ParseState<'i>, c: char) -> IResult<char> {
    // fast path
    if c.is_ascii() {
        if state.is_empty() || state.partial_string.as_bytes()[0] != c as u8 {
            Err(YError::excepted_char(c))
        }
        else {
            Ok(Parsed { term: c, rest: state.advance(1) })
        }
    }
    // utf-8 path
    else if !state.partial_string.starts_with(c) {
        Err(YError::excepted_char(c))
    }
    // not match
    else {
        // Skip a full character should be OK.
        Ok(Parsed { term: c, rest: state.advance(c) })
    }
}

pub fn parse_string_literal<'i>(state: ParseState<'i>, target: &'static str, insensitive: bool) -> IResult<'i, &'static str> {
    match state.partial_string.get(0..target.len()) {
        Some(s) if insensitive && s.eq_ignore_ascii_case(target) => Ok(Parsed { term: target, rest: state.advance(target) }),
        Some(s) if s.eq(target) => Ok(Parsed { term: target, rest: state.advance(target) }),
        _ => Err(YError::excepted_string(target)),
    }
}

#[test]
fn test() {
    let new = ParseState::new("abc");
    let result = parse_string_literal(new, "A", true);
    println!("{:?}", result)
}
