use super::*;

pub(super) fn parse_cst(input: &str, rule: BootstrapRule) -> OutputResult<BootstrapRule> {
    state(input, |state| match rule {
        BootstrapRule::Root => parse_root(state),
        BootstrapRule::Statement => parse_statement(state),
        BootstrapRule::ClassStatements => parse_class_statements(state),
        BootstrapRule::UnionStatements => parse_union_statements(state),
        BootstrapRule::NamepathFree => parse_namepath_free(state),
        BootstrapRule::Namepath => parse_namepath(state),
        BootstrapRule::Identifier => parse_identifier(state),
        BootstrapRule::Boolean => parse_boolean(state),
        BootstrapRule::KW_CLASS => parse_kw_class(state),
        BootstrapRule::KW_UNION => parse_kw_union(state),
        BootstrapRule::WhiteSpace => parse_white_space(state),
        BootstrapRule::Comment => parse_comment(state),
        BootstrapRule::IgnoreText => unreachable!(),
        BootstrapRule::IgnoreRegex => unreachable!(),
    })
}
#[inline]
fn parse_root(state: Input) -> Output {
    state.rule(BootstrapRule::Root, |s| s.repeat(0..4294967295, |s| parse_statement(s).and_then(|s| s.tag_node("statement"))))
}
#[inline]
fn parse_statement(state: Input) -> Output {
    state.rule(BootstrapRule::Statement, |s| {
        Err(s)
            .or_else(|s| parse_class_statements(s).and_then(|s| s.tag_node("class_statements")))
            .or_else(|s| parse_union_statements(s).and_then(|s| s.tag_node("union_statements")))
    })
}
#[inline]
fn parse_class_statements(state: Input) -> Output {
    state.rule(BootstrapRule::ClassStatements, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_kw_class(s))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
        })
    })
}
#[inline]
fn parse_union_statements(state: Input) -> Output {
    state.rule(BootstrapRule::UnionStatements, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_kw_union(s))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
        })
    })
}

#[inline]
fn parse_namepath_free(state: Input) -> Output {
    state.rule(BootstrapRule::NamepathFree, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| {
                                    Err(s).or_else(|s| builtin_text(s, ".", false)).or_else(|s| builtin_text(s, "::", false))
                                })
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                        })
                    })
                })
        })
    })
}

#[inline]
fn parse_namepath(state: Input) -> Output {
    state.rule(BootstrapRule::Namepath, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_text(s, "::", false))
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                        })
                    })
                })
        })
    })
}

#[inline]
fn parse_identifier(state: Input) -> Output {
    state.rule(BootstrapRule::Identifier, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^([_\\p{XID_start}]\\p{XID_continue}*)").unwrap())
        })
    })
}

#[inline]
fn parse_boolean(state: Input) -> Output {
    state.rule(BootstrapRule::Boolean, |s| {
        Err(s)
            .or_else(|s| builtin_text(s, "true", false).and_then(|s| s.tag_node("boolean_0")))
            .or_else(|s| builtin_text(s, "false", false).and_then(|s| s.tag_node("boolean_1")))
    })
}
#[inline]
fn parse_kw_class(state: Input) -> Output {
    state.rule(BootstrapRule::KW_CLASS, |s| s.match_string("class", false))
}
#[inline]
fn parse_kw_union(state: Input) -> Output {
    state.rule(BootstrapRule::KW_UNION, |s| s.match_string("union", false))
}

#[inline]
fn parse_white_space(state: Input) -> Output {
    state.rule(BootstrapRule::WhiteSpace, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(\\p{White_Space}+)").unwrap())
        })
    })
}
#[inline]
fn parse_comment(state: Input) -> Output {
    state.rule(BootstrapRule::Comment, |s| {
        Err(s).or_else(|s| {
            s.sequence(|s| {
                Ok(s)
                    .and_then(|s| {
                        builtin_regex(s, {
                            static REGEX: OnceLock<Regex> = OnceLock::new();
                            REGEX.get_or_init(|| Regex::new("^(\\)").unwrap())
                        })
                    })
                    .and_then(|s| builtin_ignore(s))
                    .and_then(|s| builtin_text(s, "/", false))
                    .and_then(|s| builtin_ignore(s))
                    .and_then(|s| {
                        s.repeat(0..4294967295, |s| {
                            builtin_regex(s, {
                                static REGEX: OnceLock<Regex> = OnceLock::new();
                                REGEX.get_or_init(|| Regex::new("^([^\\n\\r])").unwrap())
                            })
                        })
                    })
                    .and_then(|s| builtin_ignore(s))
                    .and_then(|s| {
                        builtin_regex(s, {
                            static REGEX: OnceLock<Regex> = OnceLock::new();
                            REGEX.get_or_init(|| {
                                Regex::new(
                                    "^(
    | )",
                                )
                                .unwrap()
                            })
                        })
                    })
            })
            .and_then(|s| s.tag_node("comment_0"))
        })
    })
}

/// All rules ignored in ast mode, inline is not recommended
fn builtin_ignore(state: Input) -> Output {
    state.repeat(0..u32::MAX, |s| parse_white_space(s).or_else(|s| parse_comment(s)))
}

fn builtin_any(state: Input) -> Output {
    state.rule(BootstrapRule::IgnoreText, |s| s.match_char_if(|_| true))
}

fn builtin_text<'i>(state: Input<'i>, text: &'static str, case: bool) -> Output<'i> {
    state.rule(BootstrapRule::IgnoreText, |s| s.match_string(text, case))
}

fn builtin_regex<'i, 'r>(state: Input<'i>, regex: &'r Regex) -> Output<'i> {
    state.rule(BootstrapRule::IgnoreRegex, |s| s.match_regex(regex))
}
