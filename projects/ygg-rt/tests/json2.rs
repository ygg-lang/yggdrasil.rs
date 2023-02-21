#![allow(dead_code, unused_imports, non_camel_case_types)]

use yggdrasil_rt::*;

type Input<'i> = Box<State<'i, Json5Rule>>;
type Output<'i> = Result<Box<State<'i, Json5Rule>>, Box<State<'i, Json5Rule>>>;

#[derive(Default)]
pub struct Json5Language {}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Json5Rule {
    Value,
    Object,
    Array,
    String,
    Number,
    Boolean,
    Null,
    WhiteSpace,
    /// Label for text literal
    IgnoreText,
}

impl YggdrasilRule for Json5Rule {
    fn all_rules() -> &'static [Self] {
        &[Self::Value, Self::Object, Self::Array, Self::String, Self::Number, Self::Boolean, Self::Null, Self::WhiteSpace]
    }

    fn is_ignore(&self) -> bool {
        false
    }
}

impl YggdrasilParser for Json5Language {
    type Rule = Json5Rule;
    fn parse(input: &str, rule: Json5Rule) -> OutputResult<Json5Rule> {
        state(input, |state| match rule {
            Json5Rule::Value => parse_value(state),
            Json5Rule::Object => parse_object(state),
            Json5Rule::Array => parse_array(state),
            Json5Rule::String => parse_string(state),
            Json5Rule::Number => parse_number(state),
            Json5Rule::Boolean => parse_boolean(state),
            Json5Rule::Null => parse_null(state),
            Json5Rule::WhiteSpace => parse_white_space(state),
            Json5Rule::IgnoreText => unreachable!(),
        })
    }
}
#[inline]
fn parse_value(state: Input) -> Output {
    state.rule(Json5Rule::Value, |s| {})
}
#[inline]
fn parse_object(state: Input) -> Output {
    state.rule(Json5Rule::Object, |s| {
        s.sequence(|s| {
            builtin_text(s, "{", true)
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_value(s)))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "}", true))
        })
    })
}
#[inline]
fn parse_array(state: Input) -> Output {
    state.rule(Json5Rule::Array, |s| {
        s.sequence(|s| {
            builtin_text(s, "[", true)
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.optional(|s| {
                        s.sequence(|s| {
                            parse_value(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| {
                                    s.optional(|s| {
                                        s.sequence(|s| {
                                            builtin_text(s, ",", true)
                                                .and_then(|s| builtin_ignore(s))
                                                .and_then(|s| parse_value(s))
                                        })
                                    })
                                })
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| builtin_text(s, ",", true))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "]", true))
        })
    })
}
#[inline]
fn parse_string(state: Input) -> Output {
    state.rule(Json5Rule::String, |s| parse_Bug(s))
}
#[inline]
fn parse_number(state: Input) -> Output {
    state.rule(Json5Rule::Number, |s| parse_Bug(s))
}
#[inline]
fn parse_boolean(state: Input) -> Output {
    state.rule(Json5Rule::Boolean, |s| {})
}
#[inline]
fn parse_null(state: Input) -> Output {
    state.rule(Json5Rule::Null, |s| builtin_text(s, "null", true))
}
#[inline]
fn parse_white_space(state: Input) -> Output {
    state.rule(Json5Rule::WhiteSpace, |s| parse_unicode_white_space(s))
}

/// All rules ignored in ast mode, inline is not recommended
fn builtin_ignore(state: Input) -> Output {
    Ok(state)
}

fn builtin_text<'i>(state: Input, text: &'static str, insensitive: bool) -> Output<'i> {
    state.rule(Json5Rule::IgnoreText, |s| s.match_string(text, insensitive))
}
