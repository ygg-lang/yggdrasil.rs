use yggdrasil_rt::CapturedCharacter;

use super::*;

#[derive(Debug)]
struct Output1 {
    pub a: CapturedCharacter,
    pub b: Vec<CapturedCharacter>,
    pub c: CapturedCharacter,
    pub range: std::ops::Range<usize>,
}

/// `ab{1,3}c`
fn parse_output1(state: YState) -> YResult<Output1> {
    let start = state.start_offset;
    let Parsed(state, a) = state.parse_char('a')?;
    let Parsed(state, b) = state.parse_repeats(1, 3, |state| state.parse_char('b'))?;
    let Parsed(state, c) = state.parse_char('c')?;
    let range = start..state.start_offset;
    Parsed::ok(state, Output1 { a, b, c, range })
}

#[test]
fn test_output_1() {
    println!("{:#?}", parse_output1(YState::new("ac")));
    println!("{:#?}", parse_output1(YState::new("abc")));
    println!("{:#?}", parse_output1(YState::new("abbc")));
    println!("{:#?}", parse_output1(YState::new("abbbc")));
    println!("{:#?}", parse_output1(YState::new("abbbbc")));
}
