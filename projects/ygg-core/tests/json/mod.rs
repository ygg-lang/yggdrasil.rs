#![allow(dead_code, non_camel_case_types)]

use yggdrasil_rt::{errors::YggdrasilError, iterators::TokenTree, *};

type Input<'i> = Box<State<'i, TestLanguageRule>>;
type Output<'i> = Result<Box<State<'i, TestLanguageRule>>, Box<State<'i, TestLanguageRule>>>;

#[derive(Default)]
pub struct TestLanguageParser {}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TestLanguageRule {
    Json,
    String,
    Number,
}

impl YggdrasilRule for TestLanguageRule {
    fn all_rules() -> &'static [Self] {
        &[Self::Json, Self::String, Self::Number]
    }

    fn is_ignore(&self) -> bool {
        matches!(self, Self::Json | Self::String)
    }
}

impl YggdrasilParser for TestLanguageParser {
    type Rule = TestLanguageRule;
    #[allow(clippy::almost_complete_range)]
    fn parse(rule: TestLanguageRule, input: &str) -> OutputResult<TestLanguageRule> {
        state(input, |state| match rule {
            TestLanguageRule::Json => parse_Json(state),
            TestLanguageRule::String => parse_String(state),
            TestLanguageRule::Number => parse_Number(state),
        })
    }
}

fn parse_Json(state: Input) -> Output {
    value(state)
}

fn object(state: Input) -> Output {
    state.rule(TestLanguageRule::object, |s| {
        s.sequence(|s| {
            s.match_string("{")
                .and_then(ignore)
                .and_then(pair)
                .and_then(ignore)
                .and_then(|s| {
                    s.repeat(|s| s.sequence(|s| s.match_string(",").and_then(ignore).and_then(pair).and_then(ignore)))
                })
                .and_then(|s| s.match_string("}"))
        })
        .or_else(|s| s.sequence(|s| s.match_string("{").and_then(ignore).and_then(|s| s.match_string("}"))))
    })
}

fn pair(state: Input) -> Output {
    state.rule(TestLanguageRule::pair, |s| {
        s.sequence(|s| string(s).and_then(ignore).and_then(|s| s.match_string(":")).and_then(ignore).and_then(value))
    })
}

fn array(state: Input) -> Output {
    state.rule(TestLanguageRule::array, |s| {
        s.sequence(|s| {
            s.match_string("[")
                .and_then(ignore)
                .and_then(value)
                .and_then(ignore)
                .and_then(|s| {
                    s.repeat(|s| s.sequence(|s| s.match_string(",").and_then(ignore).and_then(value).and_then(ignore)))
                })
                .and_then(|s| s.match_string("]"))
        })
        .or_else(|s| s.sequence(|s| s.match_string("[").and_then(ignore).and_then(|s| s.match_string("]"))))
    })
}

fn value(state: Input) -> Output {
    state
        .rule(TestLanguageRule::value, |s| string(s).or_else(number).or_else(object).or_else(array).or_else(bool).or_else(null))
}

fn string(state: Input) -> Output {
    state.rule(TestLanguageRule::string, |s| {
        s.match_string("\"")
            .and_then(|s| {
                s.repeat(|s| {
                    escape(s).or_else(|s| {
                        s.sequence(|s| {
                            s.lookahead(false, |s| s.match_string("\"").or_else(|s| s.match_string("\\")))
                                .and_then(|s| s.skip(1))
                        })
                    })
                })
            })
            .and_then(|pos| pos.match_string("\""))
    })
}

fn escape(state: Input) -> Output {
    state.sequence(|s| {
        s.match_string("\\").and_then(|s| {
            s.match_string("\"")
                .or_else(|s| s.match_string("\\"))
                .or_else(|s| s.match_string("/"))
                .or_else(|s| s.match_string("b"))
                .or_else(|s| s.match_string("f"))
                .or_else(|s| s.match_string("n"))
                .or_else(|s| s.match_string("r"))
                .or_else(|s| s.match_string("t"))
                .or_else(unicode)
        })
    })
}

fn unicode(state: Input) -> Output {
    state.sequence(|s| s.match_string("u").and_then(hex).and_then(hex).and_then(hex))
}

fn hex(state: Input) -> Output {
    state.match_range('0'..'9').or_else(|s| s.match_range('a'..'f')).or_else(|s| s.match_range('A'..'F'))
}

fn number(state: Input) -> Output {
    state.rule(TestLanguageRule::number, |s| {
        s.sequence(|s| {
            s.optional(|s| s.match_string("-")).and_then(int).and_then(|s| {
                s.optional(|s| {
                    s.sequence(|s| {
                        s.match_string(".")
                            .and_then(|s| s.match_range('0'..'9'))
                            .and_then(|s| s.repeat(|s| s.match_range('0'..'9')))
                            .and_then(|s| s.optional(exp))
                            .or_else(exp)
                    })
                })
            })
        })
    })
}

fn int(state: Input) -> Output {
    state
        .match_string("0")
        .or_else(|s| s.sequence(|s| s.match_range('1'..'9').and_then(|s| s.repeat(|s| s.match_range('0'..'9')))))
}

fn exp(state: Input) -> Output {
    state.sequence(|s| {
        s.match_string("E")
            .or_else(|s| s.match_string("e"))
            .and_then(|s| s.optional(|s| s.match_string("+").or_else(|s| s.match_string("-"))))
            .and_then(int)
    })
}

fn bool(state: Input) -> Output {
    state.rule(TestLanguageRule::bool, |s| s.match_string("true").or_else(|s| s.match_string("false")))
}

fn null(state: Input) -> Output {
    state.rule(TestLanguageRule::null, |s| s.match_string("null"))
}

fn ignore(state: Input) -> Output {
    state.repeat(|s| {
        s.match_string(" ")
            .or_else(|s| s.match_string("\t"))
            .or_else(|s| s.match_string("\r"))
            .or_else(|s| s.match_string("\n"))
    })
}
