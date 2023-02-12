use std::collections::HashMap;
use yggdrasil_rt::{
    consumes_to,
    error::YggdrasilError,
    iterators::{Pair, TokenTree},
    parses_to, state, State, TextSpan, YggdrasilParser, YggdrasilRule,
};

type Input<'i> = Box<State<'i, JsonRule>>;
type Output<'i> = Result<Box<State<'i, JsonRule>>, Box<State<'i, JsonRule>>>;

#[derive(Default)]
pub struct JsonParser {}

#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum JsonRule {

    json,
    object,
    pair,
    array,
    value,
    string,
    escape,
    unicode,
    hex,
    number,
    int,
    exp,
    bool,
    null,
}

impl YggdrasilRule for JsonRule {
    fn all_rules() -> &'static [Self] {
        &[Self::json]
    }

    fn is_ignore(&self) -> bool {
        matches!(self, Self::json | Self::object)
    }
}

impl YggdrasilParser for JsonParser {
    type Rule = JsonRule;
    #[allow(clippy::almost_complete_range)]
    fn parse(rule: JsonRule, input: &str) -> Result<TokenTree<JsonRule>, YggdrasilError<JsonRule>> {
        state(input, |state| match rule {
            JsonRule::json => json(state),
            JsonRule::object => object(state),
            JsonRule::pair => pair(state),
            JsonRule::array => array(state),
            JsonRule::value => value(state),
            JsonRule::string => string(state),
            JsonRule::escape => escape(state),
            JsonRule::unicode => unicode(state),
            JsonRule::hex => hex(state),
            JsonRule::number => number(state),
            JsonRule::int => int(state),
            JsonRule::exp => exp(state),
            JsonRule::bool => bool(state),
            JsonRule::null => null(state),
        })
    }
}

fn json(state: Input) -> Output {
    value(state)
}

fn object(state: Input) -> Output {
    state.rule(JsonRule::object, |s| {
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
    state.rule(JsonRule::pair, |s| {
        s.sequence(|s| string(s).and_then(ignore).and_then(|s| s.match_string(":")).and_then(ignore).and_then(value))
    })
}

fn array(state: Input) -> Output {
    state.rule(JsonRule::array, |s| {
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
    state.rule(JsonRule::value, |s| string(s).or_else(number).or_else(object).or_else(array).or_else(bool).or_else(null))
}

fn string(state: Input) -> Output {
    state.rule(JsonRule::string, |s| {
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
    state.rule(JsonRule::number, |s| {
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
    state.rule(JsonRule::bool, |s| s.match_string("true").or_else(|s| s.match_string("false")))
}

fn null(state: Input) -> Output {
    state.rule(JsonRule::null, |s| s.match_string("null"))
}

fn ignore(state: Input) -> Output {
    state.repeat(|s| {
        s.match_string(" ")
            .or_else(|s| s.match_string("\t"))
            .or_else(|s| s.match_string("\r"))
            .or_else(|s| s.match_string("\n"))
    })
}