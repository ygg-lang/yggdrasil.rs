use super::*;

pub fn parse_char(state: ParseState, target: char) -> IResult<char> {
    match state.get_character(0) {
        Some(c) if c.eq(&target) => Ok(Parsed { term: target, rest: state.advance(target) }),
        _ => Err(IError::excepted_character(target)),
    }
}

pub fn parse_char_range(state: ParseState, start: char, end: char) -> IResult<char> {
    match state.get_character(0) {
        Some(c) if c >= start && c <= end => Ok(Parsed { term: c, rest: state.advance(c) }),
        _ => Err(IError::excepted_character_range(start, end)),
    }
}

pub fn parse_string_literal<'i>(state: ParseState<'i>, target: &'static str, insensitive: bool) -> IResult<'i, &'static str> {
    match state.get_string(0..target.len()) {
        Some(s) if insensitive && s.eq_ignore_ascii_case(target) => Ok(Parsed { term: target, rest: state.advance(target) }),
        Some(s) if s.eq(target) => Ok(Parsed { term: target, rest: state.advance(target) }),
        _ => Err(IError::excepted_string(target)),
    }
}

#[test]
fn test() {
    let new = ParseState::new("abc");
    let result = parse_string_literal(new, "A", true);
    println!("{:?}", result)
}
