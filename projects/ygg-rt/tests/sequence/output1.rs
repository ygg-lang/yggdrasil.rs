use super::*;

#[derive(Debug)]
struct Output1 {
    pub a: char,
    pub b: Vec<char>,
    pub c: char,
    pub range: std::ops::Range<usize>,
}

/// `ab{1,3}c`
fn parse_output1(state: YState, min: usize, max: usize) -> YResult<Output1> {
    let start = state.start_offset;
    let (state, a) = state.match_char('a')?;
    let (state, b) = state.match_repeat_m_n(min, max, |state| state.match_char('b'))?;
    let (state, c) = state.match_char('c')?;
    let range = start..state.start_offset;
    state.finish(Output1 { a, b, c, range })
}

#[test]
fn test_output_02() {
    println!("{:#?}", parse_output1(YState::new("ac"), 0, 2));
    println!("{:#?}", parse_output1(YState::new("abc"), 0, 2));
    println!("{:#?}", parse_output1(YState::new("abbc"), 0, 2));
    println!("{:#?}", parse_output1(YState::new("abbbc"), 0, 2));
}

#[test]
fn test_output_13() {
    println!("{:#?}", parse_output1(YState::new("ac"), 1, 3));
    println!("{:#?}", parse_output1(YState::new("abc"), 1, 3));
    println!("{:#?}", parse_output1(YState::new("abbc"), 1, 3));
    println!("{:#?}", parse_output1(YState::new("abbbc"), 1, 3));
    println!("{:#?}", parse_output1(YState::new("abbbbc"), 1, 3));
}
