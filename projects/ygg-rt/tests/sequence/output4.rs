use super::*;

/// `a{1,2}b|a{1,3}c`
#[derive(Debug)]
enum Output4 {
    Output4Aux1 { a: Vec<char>, b: char },
    Output4Aux2 { a: Vec<char>, c: char },
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
    let (state, value) = state.begin_choice().maybe(parse_output3_aux2).maybe(parse_output3_aux3).end_choice()?;
    state.finish(value)
}

/// `a{1,2}b`
fn parse_output3_aux2(state: YState) -> YResult<Output4> {
    let (state, a) = state.match_repeat_m_n(1, 2, |state| state.parse_char('a'))?;
    let (state, b) = state.parse_char('b')?;
    state.finish(Output4::Output4Aux1 { a, b })
}

/// `a{1,3}c`
fn parse_output3_aux3(state: YState) -> YResult<Output4> {
    let (state, a) = state.match_repeat_m_n(1, 3, |state| state.parse_char('a'))?;
    let (state, c) = state.parse_char('c')?;
    state.finish(Output4::Output4Aux2 { a, c })
}
