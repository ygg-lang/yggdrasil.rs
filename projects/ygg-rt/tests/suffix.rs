use yggdrasil_rt::{Either, State, YggdrasilRule};

impl YggdrasilRule for Rule {}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Rule {
    a,
}

#[test]
fn test() {
    assert!(repeat_a("b").is_ok());
    assert!(repeat_a("ab").is_ok());
    assert!(repeat_a("aab").is_ok());
    assert!(repeat_a("aaab").is_err());
}

fn repeat_a(input: &str) -> Either<Box<State<Rule>>> {
    let state: Box<State<Rule>> = State::new(input);
    state.repeat(0..2, |s| s.match_string("a", false))
}
