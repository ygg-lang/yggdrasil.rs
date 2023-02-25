#![allow(dead_code, unused_imports, non_camel_case_types)]
#![doc = include_str!("readme.md")]

use std::sync::OnceLock;
use yggdrasil_rt::*;

type Input<'i> = Box<State<'i, Json5Rule>>;
type Output<'i> = Result<Box<State<'i, Json5Rule>>, Box<State<'i, Json5Rule>>>;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Json5Language {}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Json5Rule {
    Value,
    Object,
    ObjectPair,
    Array,
    String,
    StringEscaped,
    Number,
    Boolean,
    Null,
    Identifier,
    WhiteSpace,
    /// Label for text literal
    IgnoreText,
    /// Label for regex literal
    IgnoreRegex,
}

impl YggdrasilRule for Json5Rule {
    fn all_rules() -> &'static [Self] {
        &[
            Self::Value,
            Self::Object,
            Self::ObjectPair,
            Self::Array,
            Self::String,
            Self::StringEscaped,
            Self::Number,
            Self::Boolean,
            Self::Null,
            Self::Identifier,
            Self::WhiteSpace,
        ]
    }

    fn is_ignore(&self) -> bool {
        matches!(self, Self::IgnoreText | Self::IgnoreRegex | Self::WhiteSpace)
    }
}

// ===================================================
use super::*;

impl YggdrasilParser for Json5Language {
    type Rule = Json5Rule;
    fn parse(input: &str, rule: Json5Rule) -> OutputResult<Json5Rule> {
        state(input, |state| match rule {
            Json5Rule::Value => parse_value(state),
            Json5Rule::Object => parse_object(state),
            Json5Rule::ObjectPair => parse_object_pair(state),
            Json5Rule::Array => parse_array(state),
            Json5Rule::String => parse_string(state),
            Json5Rule::StringEscaped => parse_string_escaped(state),
            Json5Rule::Number => parse_number(state),
            Json5Rule::Boolean => parse_boolean(state),
            Json5Rule::Null => parse_null(state),
            Json5Rule::Identifier => parse_identifier(state),
            Json5Rule::WhiteSpace => parse_white_space(state),
            Json5Rule::IgnoreText => unreachable!(),
            Json5Rule::IgnoreRegex => unreachable!(),
        })
    }
}
#[inline]
fn parse_value(state: Input) -> Output {
    state.rule(Json5Rule::Value, |s| {
        parse_object(s)
            .or_else(|s| parse_array(s))
            .or_else(|s| parse_string(s))
            .or_else(|s| parse_number(s))
            .or_else(|s| parse_boolean(s))
            .or_else(|s| parse_null(s))
    })
}
#[inline]
fn parse_object(state: Input) -> Output {
    state.rule(Json5Rule::Object, |s| {
        s.sequence(|s| {
            builtin_text::<false>(s, "{")
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.optional(|s| {
                        s.sequence(|s| {
                            parse_object_pair(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| {
                                    s.optional(|s| {
                                        s.sequence(|s| {
                                            builtin_text::<false>(s, ",")
                                                .and_then(|s| builtin_ignore(s))
                                                .and_then(|s| parse_object_pair(s))
                                        })
                                    })
                                })
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| builtin_text::<false>(s, ","))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text::<false>(s, "}"))
        })
    })
}
#[inline]
fn parse_object_pair(state: Input) -> Output {
    state.rule(Json5Rule::ObjectPair, |s| {
        s.sequence(|s| parse_identifier(s).and_then(|s| builtin_text::<false>(s, ":")).and_then(|s| parse_value(s))).or_else(
            |s| s.sequence(|s| parse_string(s).and_then(|s| builtin_text::<false>(s, ":")).and_then(|s| parse_value(s))),
        )
    })
}
#[inline]
fn parse_array(state: Input) -> Output {
    state.rule(Json5Rule::Array, |s| {
        s.sequence(|s| {
            builtin_text::<false>(s, "[")
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.optional(|s| {
                        s.sequence(|s| {
                            parse_value(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| {
                                    s.optional(|s| {
                                        s.sequence(|s| {
                                            builtin_text::<false>(s, ",")
                                                .and_then(|s| builtin_ignore(s))
                                                .and_then(|s| parse_value(s))
                                        })
                                    })
                                })
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| builtin_text::<false>(s, ","))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text::<false>(s, "]"))
        })
    })
}
#[inline]
fn parse_string(state: Input) -> Output {
    state.rule(Json5Rule::String, |s| {
        s.sequence(|s| {
            builtin_text::<false>(s, "'")
                .and_then(|s| builtin_text::<false>(s, "'"))
                .and_then(|s| builtin_text::<false>(s, "'"))
                .and_then(|s| parse_string_escaped(s))
                .and_then(|s| s.match_char_if(|_| true))
                .and_then(|s| builtin_text::<false>(s, "'"))
        })
    })
}
#[inline]
fn parse_string_escaped(state: Input) -> Output {
    state.rule(Json5Rule::StringEscaped, |s| {
        s.sequence(|s| builtin_text::<false>(s, "\\").and_then(|s| s.match_char_if(|_| true)))
    })
}
#[inline]
fn parse_number(state: Input) -> Output {
    state.rule(Json5Rule::Number, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^([+-]?(0|[1-9][0-9]*))").unwrap())
        })
    })
}
#[inline]
fn parse_boolean(state: Input) -> Output {
    state.rule(Json5Rule::Boolean, |s| builtin_text::<false>(s, "true").or_else(|s| builtin_text::<false>(s, "false")))
}
#[inline]
fn parse_null(state: Input) -> Output {
    state.rule(Json5Rule::Null, |s| builtin_text::<false>(s, "null"))
}
#[inline]
fn parse_identifier(state: Input) -> Output {
    state.rule(Json5Rule::Identifier, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^([_\\p{XID_start}][\\p{XID_continue}]*)").unwrap())
        })
    })
}
#[inline]
fn parse_white_space(state: Input) -> Output {
    state.rule(Json5Rule::WhiteSpace, |s| {
        s.sequence(|s| {
            builtin_text::<false>(s, " ")
                .and_then(|s| builtin_text::<false>(s, "\\n"))
                .and_then(|s| builtin_text::<false>(s, "\\r"))
        })
    })
}

/// All rules ignored in ast mode, inline is not recommended
fn builtin_ignore(state: Input) -> Output {
    state.repeat(|s| parse_white_space(s))
}

fn builtin_text<'i, const INSENSITIVE: bool>(state: Input<'i>, text: &'static str) -> Output<'i> {
    state.rule(Json5Rule::IgnoreText, |s| s.match_string::<INSENSITIVE>(text))
}

fn builtin_regex<'i, 'r>(state: Input<'i>, regex: &'r Regex) -> Output<'i> {
    state.rule(Json5Rule::IgnoreRegex, |s| s.match_regex(regex))
}
