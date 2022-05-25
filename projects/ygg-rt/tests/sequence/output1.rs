use super::*;

#[derive(Debug)]
struct Output1 {
    a: char,
    b: Vec<char>,
    c: char,
}

/// `ab{1,3}c`
fn parse_output1(state: YState) -> YResult<Output1> {
    let Parsed { value: a, state } = state.parse_char('a')?;
    let Parsed { value: b, state } = state.parse_repeats(1, 3, |state| state.parse_char('b'))?;
    let Parsed { value: c, state } = state.parse_char('c')?;
    Parsed::ok(Output1 { a, b, c }, state)
}

#[test]
fn test_output_1() {
    println!("{:#?}", parse_output1(YState::new("ac")));
    println!("{:#?}", parse_output1(YState::new("abc")));
    println!("{:#?}", parse_output1(YState::new("abbc")));
    println!("{:#?}", parse_output1(YState::new("abbbc")));
    println!("{:#?}", parse_output1(YState::new("abbbbc")));
}
