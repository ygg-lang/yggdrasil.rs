use super::*;

/// `a{1,2}b|a{1,3}c`
#[derive(Debug)]
struct Output3 {
    a: Vec<char>,
    b: Option<char>,
    c: Option<char>,
}

enum Output3Aux1 {
    Output3Aux2 { a: Vec<char>, b: char },
    Output3Aux3 { a: Vec<char>, c: char },
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
fn parse_output3(state: YState) -> YResult<Output3> {
    let Parsed(state, value) = state.start_choice().or_else(parse_output3_aux2).or_else(parse_output3_aux3).end_choice()?;
    let value = match value {
        Output3Aux1::Output3Aux2 { a, b } => Output3 { a, b: Some(b), c: None },
        Output3Aux1::Output3Aux3 { a, c } => Output3 { a, b: None, c: Some(c) },
    };
    Parsed::ok(state, value)
}

/// `a{1,2}b`
fn parse_output3_aux2(state: YState) -> YResult<Output3Aux1> {
    let Parsed(state, a) = state.parse_repeats(1, 2, |state| state.parse_char('a'))?;
    let Parsed(state, b) = state.parse_char('b')?;
    Parsed::ok(state, Output3Aux1::Output3Aux2 { a, b })
}

/// `a{1,3}c`
fn parse_output3_aux3(state: YState) -> YResult<Output3Aux1> {
    let Parsed(state, a) = state.parse_repeats(1, 3, |state| state.parse_char('a'))?;
    let Parsed(state, c) = state.parse_char('c')?;
    Parsed::ok(state, Output3Aux1::Output3Aux3 { a, c })
}
