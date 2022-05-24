use super::*;

/// `a{1,2}b|a{1,3}c`
#[derive(Debug)]
enum Output4 {
    Output4Aux1 { a: Vec<char>, b: char },
    Output4Aux2 { a: Vec<char>, c: char },
}

#[test]
fn test_output_1() {
    println!("{:#?}", parse_output3(ParseState::new("ac")));
    println!("{:#?}", parse_output3(ParseState::new("abc")));
    println!("{:#?}", parse_output3(ParseState::new("abbc")));
    println!("{:#?}", parse_output3(ParseState::new("abbbc")));
    println!("{:#?}", parse_output3(ParseState::new("abbbbc")));
}

/// `ab{1,3}c`
fn parse_output3(state: ParseState) -> IResult<Output4> {
    let Parsed { value, state } = state.start_choice().or_else(parse_output3_aux2).or_else(parse_output3_aux3).end_choice()?;
    Parsed::ok(value, state)
}

/// `a{1,2}b`
fn parse_output3_aux2(state: ParseState) -> IResult<Output4> {
    let Parsed { value: a, state } = state.parse_repeats(1, 2, |state| state.parse_char('a'))?;
    let Parsed { value: b, state } = state.parse_char('b')?;
    Parsed::ok(Output4::Output4Aux1 { a, b }, state)
}

/// `a{1,3}c`
fn parse_output3_aux3(state: ParseState) -> IResult<Output4> {
    let Parsed { value: a, state } = state.parse_repeats(1, 3, |state| state.parse_char('a'))?;
    let Parsed { value: c, state } = state.parse_char('c')?;
    Parsed::ok(Output4::Output4Aux2 { a, c }, state)
}
