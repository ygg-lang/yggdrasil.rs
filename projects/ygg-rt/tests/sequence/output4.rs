use super::*;

/// `a{1,2}b|a{1,3}c`
#[derive(Debug)]
enum Output4 {
    Output4Aux1 { a: Vec<CapturedCharacter>, b: CapturedCharacter },
    Output4Aux2 { a: Vec<CapturedCharacter>, c: CapturedCharacter },
}

#[test]
fn test_output_1() {
    println!("{:#?}", parse_output3(YState::new("ac")));
    println!("{:#?}", parse_output3(YState::new("abc")));
    println!("{:#?}", parse_output3(YState::new("abbc")));
    println!("{:#?}", parse_output3(YState::new("abbbc")));
    println!("{:#?}", parse_output3(YState::new("abbbbc")));
}

/// `ab{1,3}c`
fn parse_output3(state: YState) -> YResult<Output4> {
    let Parsed(state, value) = state.start_choice().or_else(parse_output3_aux2).or_else(parse_output3_aux3).end_choice()?;
    Parsed::ok(state, value)
}

/// `a{1,2}b`
fn parse_output3_aux2(state: YState) -> YResult<Output4> {
    let Parsed(state, a) = state.parse_repeats(1, 2, |state| state.parse_char('a'))?;
    let Parsed(state, b) = state.parse_char('b')?;
    Parsed::ok(state, Output4::Output4Aux1 { a, b })
}

/// `a{1,3}c`
fn parse_output3_aux3(state: YState) -> YResult<Output4> {
    let Parsed(state, a) = state.parse_repeats(1, 3, |state| state.parse_char('a'))?;
    let Parsed(state, c) = state.parse_char('c')?;
    Parsed::ok(state, Output4::Output4Aux2 { a, c })
}
