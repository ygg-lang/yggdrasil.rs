use super::*;

pub(super) fn parse_cst(input: &str, rule: BootstrapRule) -> OutputResult<BootstrapRule> {
    state(input, |state| match rule {
        BootstrapRule::Root => parse_root(state),
        BootstrapRule::Statement => parse_statement(state),
        BootstrapRule::GrammarStatement => parse_grammar_statement(state),
        BootstrapRule::GrammarTerm1 => parse_grammar_term_1(state),
        BootstrapRule::GrammarTerm => parse_grammar_term(state),
        BootstrapRule::GrammarPair => parse_grammar_pair(state),
        BootstrapRule::GrammarValue => parse_grammar_value(state),
        BootstrapRule::GrammarDict => parse_grammar_dict(state),
        BootstrapRule::GrammarList => parse_grammar_list(state),
        BootstrapRule::GrammarListTerms => parse_grammar_list_terms(state),
        BootstrapRule::ClassStatement => parse_class_statement(state),
        BootstrapRule::ClassBlock => parse_class_block(state),
        BootstrapRule::OP_REMARK => parse_op_remark(state),
        BootstrapRule::UnionStatement => parse_union_statement(state),
        BootstrapRule::UnionBlock => parse_union_block(state),
        BootstrapRule::UnionBranch => parse_union_branch(state),
        BootstrapRule::BranchTag => parse_branch_tag(state),
        BootstrapRule::RightAssociativity => parse_right_associativity(state),
        BootstrapRule::GroupStatement => parse_group_statement(state),
        BootstrapRule::GroupBlock => parse_group_block(state),
        BootstrapRule::GroupPair => parse_group_pair(state),
        BootstrapRule::DecoratorCall => parse_decorator_call(state),
        BootstrapRule::DecoratorName => parse_decorator_name(state),
        BootstrapRule::FunctionCall => parse_function_call(state),
        BootstrapRule::FunctionName => parse_function_name(state),
        BootstrapRule::CallBody => parse_call_body(state),
        BootstrapRule::Expression => parse_expression(state),
        BootstrapRule::ExpressionHard => parse_expression_hard(state),
        BootstrapRule::ExpressionSoft => parse_expression_soft(state),
        BootstrapRule::ExpressionTag => parse_expression_tag(state),
        BootstrapRule::Term => parse_term(state),
        BootstrapRule::Negative => parse_negative(state),
        BootstrapRule::Positive => parse_positive(state),
        BootstrapRule::Remark => parse_remark(state),
        BootstrapRule::Prefix => parse_prefix(state),
        BootstrapRule::Optional => parse_optional(state),
        BootstrapRule::Many => parse_many(state),
        BootstrapRule::Many1 => parse_many_1(state),
        BootstrapRule::Suffix => parse_suffix(state),
        BootstrapRule::Atomic => parse_atomic(state),
        BootstrapRule::GroupExpression => parse_group_expression(state),
        BootstrapRule::StringRaw => parse_string_raw(state),
        BootstrapRule::StringRawText => parse_string_raw_text(state),
        BootstrapRule::StringNormal => parse_string_normal(state),
        BootstrapRule::StringItem => parse_string_item(state),
        BootstrapRule::EscapedUnicode => parse_escaped_unicode(state),
        BootstrapRule::EscapedCharacter => parse_escaped_character(state),
        BootstrapRule::HEX => parse_hex(state),
        BootstrapRule::TextAny => parse_text_any(state),
        BootstrapRule::RegexEmbed => parse_regex_embed(state),
        BootstrapRule::RegexItem => parse_regex_item(state),
        BootstrapRule::RegexCharacter => parse_regex_character(state),
        BootstrapRule::RegexRange => parse_regex_range(state),
        BootstrapRule::RegexNegative => parse_regex_negative(state),
        BootstrapRule::Category => parse_category(state),
        BootstrapRule::NamepathFree => parse_namepath_free(state),
        BootstrapRule::Namepath => parse_namepath(state),
        BootstrapRule::Identifier => parse_identifier(state),
        BootstrapRule::True => parse_true(state),
        BootstrapRule::False => parse_false(state),
        BootstrapRule::Boolean => parse_boolean(state),
        BootstrapRule::Integer => parse_integer(state),
        BootstrapRule::RangeExact => parse_range_exact(state),
        BootstrapRule::Range => parse_range(state),
        BootstrapRule::ModifierCall => parse_modifier_call(state),
        BootstrapRule::OP_CATEGORY => parse_op_category(state),
        BootstrapRule::Parser => parse_parser(state),
        BootstrapRule::Inspector => parse_inspector(state),
        BootstrapRule::External => parse_external(state),
        BootstrapRule::KW_EXTERNAL => parse_kw_external(state),
        BootstrapRule::KW_GRAMMAR => parse_kw_grammar(state),
        BootstrapRule::KW_IMPORT => parse_kw_import(state),
        BootstrapRule::KW_CLASS => parse_kw_class(state),
        BootstrapRule::KW_UNION => parse_kw_union(state),
        BootstrapRule::KW_GROUP => parse_kw_group(state),
        BootstrapRule::KW_CLIMB => parse_kw_climb(state),
        BootstrapRule::KW_MACRO => parse_kw_macro(state),
        BootstrapRule::WhiteSpace => parse_white_space(state),
        BootstrapRule::Comment => parse_comment(state),
        BootstrapRule::HiddenText => unreachable!(),
    })
}
#[inline]
fn parse_root(state: Input) -> Output {
    state.rule(BootstrapRule::Root, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_statement(s).and_then(|s| s.tag_node("statement")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.end_of_input())
        })
    })
}
#[inline]
fn parse_statement(state: Input) -> Output {
    state.rule(BootstrapRule::Statement, |s| {
        Err(s)
            .or_else(|s| parse_grammar_statement(s).and_then(|s| s.tag_node("grammar_statement")))
            .or_else(|s| parse_class_statement(s).and_then(|s| s.tag_node("class_statement")))
            .or_else(|s| parse_union_statement(s).and_then(|s| s.tag_node("union_statement")))
            .or_else(|s| parse_group_statement(s).and_then(|s| s.tag_node("group_statement")))
    })
}
#[inline]
fn parse_grammar_statement(state: Input) -> Output {
    state.rule(BootstrapRule::GrammarStatement, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_kw_grammar(s))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_grammar_dict(s).and_then(|s| s.tag_node("grammar_dict")))
        })
    })
}
#[inline]
fn parse_grammar_term_1(state: Input) -> Output {
    state.rule(BootstrapRule::GrammarTerm1, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)([,;])").unwrap())
        })
    })
}
#[inline]
fn parse_grammar_term(state: Input) -> Output {
    state.rule(BootstrapRule::GrammarTerm, |s| {
        Err(s)
            .or_else(|s| parse_grammar_pair(s).and_then(|s| s.tag_node("grammar_pair")))
            .or_else(|s| parse_grammar_term_1(s).and_then(|s| s.tag_node("grammar_term_1")))
    })
}
#[inline]
fn parse_grammar_pair(state: Input) -> Output {
    state.rule(BootstrapRule::GrammarPair, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("key")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, ":", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_grammar_value(s).and_then(|s| s.tag_node("grammar_value")))
        })
    })
}
#[inline]
fn parse_grammar_value(state: Input) -> Output {
    state.rule(BootstrapRule::GrammarValue, |s| {
        Err(s)
            .or_else(|s| parse_grammar_dict(s).and_then(|s| s.tag_node("grammar_dict")))
            .or_else(|s| parse_grammar_list(s).and_then(|s| s.tag_node("grammar_list")))
            .or_else(|s| parse_namepath(s).and_then(|s| s.tag_node("namepath")))
            .or_else(|s| parse_string_raw(s).and_then(|s| s.tag_node("string_raw")))
            .or_else(|s| parse_string_normal(s).and_then(|s| s.tag_node("string_normal")))
    })
}
#[inline]
fn parse_grammar_dict(state: Input) -> Output {
    state.rule(BootstrapRule::GrammarDict, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "{", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_grammar_term(s).and_then(|s| s.tag_node("grammar_term")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "}", false))
        })
    })
}
#[inline]
fn parse_grammar_list(state: Input) -> Output {
    state.rule(BootstrapRule::GrammarList, |s| {
        Err(s)
            .or_else(|s| {
                s.sequence(|s| {
                    Ok(s)
                        .and_then(|s| builtin_text(s, "(", false))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| {
                            s.optional(|s| {
                                s.sequence(|s| {
                                    Ok(s)
                                        .and_then(|s| parse_grammar_value(s).and_then(|s| s.tag_node("grammar_value")))
                                        .and_then(|s| builtin_ignore(s))
                                        .and_then(|s| {
                                            s.repeat(0..4294967295, |s| {
                                                s.sequence(|s| {
                                                    Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
                                                        s.sequence(|s| {
                                                            Ok(s)
                                                                .and_then(|s| builtin_text(s, ",", false))
                                                                .and_then(|s| builtin_ignore(s))
                                                                .and_then(|s| {
                                                                    parse_grammar_value(s)
                                                                        .and_then(|s| s.tag_node("grammar_value"))
                                                                })
                                                        })
                                                    })
                                                })
                                            })
                                        })
                                        .and_then(|s| builtin_ignore(s))
                                        .and_then(|s| s.optional(|s| builtin_text(s, ",", false)))
                                })
                            })
                        })
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| builtin_text(s, ")", false))
                })
            })
            .or_else(|s| {
                s.sequence(|s| {
                    Ok(s)
                        .and_then(|s| builtin_text(s, "[", false))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| {
                            s.optional(|s| {
                                s.sequence(|s| {
                                    Ok(s)
                                        .and_then(|s| parse_grammar_value(s).and_then(|s| s.tag_node("grammar_value")))
                                        .and_then(|s| builtin_ignore(s))
                                        .and_then(|s| {
                                            s.repeat(0..4294967295, |s| {
                                                s.sequence(|s| {
                                                    Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
                                                        s.sequence(|s| {
                                                            Ok(s)
                                                                .and_then(|s| builtin_text(s, ",", false))
                                                                .and_then(|s| builtin_ignore(s))
                                                                .and_then(|s| {
                                                                    parse_grammar_value(s)
                                                                        .and_then(|s| s.tag_node("grammar_value"))
                                                                })
                                                        })
                                                    })
                                                })
                                            })
                                        })
                                        .and_then(|s| builtin_ignore(s))
                                        .and_then(|s| s.optional(|s| builtin_text(s, ",", false)))
                                })
                            })
                        })
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| builtin_text(s, "]", false))
                })
            })
    })
}
#[inline]
fn parse_grammar_list_terms(state: Input) -> Output {
    state.rule(BootstrapRule::GrammarListTerms, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_grammar_value(s).and_then(|s| s.tag_node("grammar_value")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
                                s.sequence(|s| {
                                    Ok(s)
                                        .and_then(|s| builtin_text(s, ",", false))
                                        .and_then(|s| builtin_ignore(s))
                                        .and_then(|s| parse_grammar_value(s).and_then(|s| s.tag_node("grammar_value")))
                                })
                            })
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| builtin_text(s, ",", false)))
        })
    })
}
#[inline]
fn parse_class_statement(state: Input) -> Output {
    state.rule(BootstrapRule::ClassStatement, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.sequence(|s| {
                        Ok(s)
                            .and_then(|s| {
                                s.repeat(0..4294967295, |s| {
                                    s.sequence(|s| {
                                        Ok(s)
                                            .and_then(|s| builtin_ignore(s))
                                            .and_then(|s| parse_decorator_call(s).and_then(|s| s.tag_node("decorator_call")))
                                    })
                                })
                            })
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| {
                                s.repeat(0..4294967295, |s| {
                                    s.sequence(|s| {
                                        Ok(s)
                                            .and_then(|s| builtin_ignore(s))
                                            .and_then(|s| parse_modifier_call(s).and_then(|s| s.tag_node("modifier_call")))
                                    })
                                })
                            })
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_kw_class(s))
                            .and_then(|s| builtin_ignore(s))
                    })
                })
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("name")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.optional(|s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| {
                                    s.sequence(|s| {
                                        Ok(s).and_then(|s| builtin_text(s, "->", false)).and_then(|s| builtin_ignore(s))
                                    })
                                })
                                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("cast")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_op_remark(s).and_then(|s| s.tag_node("op_remark"))))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_class_block(s).and_then(|s| s.tag_node("class_block")))
        })
    })
}
#[inline]
fn parse_class_block(state: Input) -> Output {
    state.rule(BootstrapRule::ClassBlock, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "{", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| builtin_text(s, "|", false)))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_expression(s).and_then(|s| s.tag_node("expression")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "}", false))
        })
    })
}
#[inline]
fn parse_op_remark(state: Input) -> Output {
    state.rule(BootstrapRule::OP_REMARK, |s| s.match_string("^", false))
}
#[inline]
fn parse_union_statement(state: Input) -> Output {
    state.rule(BootstrapRule::UnionStatement, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.sequence(|s| {
                        Ok(s)
                            .and_then(|s| {
                                s.repeat(0..4294967295, |s| {
                                    s.sequence(|s| {
                                        Ok(s)
                                            .and_then(|s| builtin_ignore(s))
                                            .and_then(|s| parse_decorator_call(s).and_then(|s| s.tag_node("decorator_call")))
                                    })
                                })
                            })
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| {
                                s.repeat(0..4294967295, |s| {
                                    s.sequence(|s| {
                                        Ok(s)
                                            .and_then(|s| builtin_ignore(s))
                                            .and_then(|s| parse_modifier_call(s).and_then(|s| s.tag_node("modifier_call")))
                                    })
                                })
                            })
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_kw_union(s))
                            .and_then(|s| builtin_ignore(s))
                    })
                })
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("name")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_op_remark(s).and_then(|s| s.tag_node("op_remark"))))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_union_block(s).and_then(|s| s.tag_node("union_block")))
        })
    })
}
#[inline]
fn parse_union_block(state: Input) -> Output {
    state.rule(BootstrapRule::UnionBlock, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "{", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_union_branch(s).and_then(|s| s.tag_node("union_branch")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "}", false))
        })
    })
}
#[inline]
fn parse_union_branch(state: Input) -> Output {
    state.rule(BootstrapRule::UnionBranch, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "|", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_expression_hard(s).and_then(|s| s.tag_node("expression_hard")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_branch_tag(s).and_then(|s| s.tag_node("branch_tag"))))
        })
    })
}
#[inline]
fn parse_branch_tag(state: Input) -> Output {
    state.rule(BootstrapRule::BranchTag, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "#", false))
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                .and_then(|s| s.optional(|s| parse_right_associativity(s).and_then(|s| s.tag_node("right_associativity"))))
        })
    })
}
#[inline]
fn parse_right_associativity(state: Input) -> Output {
    state.rule(BootstrapRule::RightAssociativity, |s| s.match_string(">", false))
}
#[inline]
fn parse_group_statement(state: Input) -> Output {
    state.rule(BootstrapRule::GroupStatement, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_decorator_call(s).and_then(|s| s.tag_node("decorator_call")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_modifier_call(s).and_then(|s| s.tag_node("modifier_call")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_kw_group(s))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier"))))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_group_block(s).and_then(|s| s.tag_node("group_block")))
        })
    })
}
#[inline]
fn parse_group_block(state: Input) -> Output {
    state.rule(BootstrapRule::GroupBlock, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "{", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_group_pair(s).and_then(|s| s.tag_node("group_pair")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "}", false))
        })
    })
}
#[inline]
fn parse_group_pair(state: Input) -> Output {
    state.rule(BootstrapRule::GroupPair, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, ":", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_atomic(s).and_then(|s| s.tag_node("atomic")))
        })
    })
}
#[inline]
fn parse_decorator_call(state: Input) -> Output {
    state.rule(BootstrapRule::DecoratorCall, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_decorator_name(s).and_then(|s| s.tag_node("decorator_name")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_call_body(s).and_then(|s| s.tag_node("call_body")))
        })
    })
}
#[inline]
fn parse_decorator_name(state: Input) -> Output {
    state.rule(BootstrapRule::DecoratorName, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    builtin_regex(s, {
                        static REGEX: OnceLock<Regex> = OnceLock::new();
                        REGEX.get_or_init(|| Regex::new("^(?x)([@\\#])").unwrap())
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
        })
    })
}
#[inline]
fn parse_function_call(state: Input) -> Output {
    state.rule(BootstrapRule::FunctionCall, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_function_name(s).and_then(|s| s.tag_node("function_name")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_call_body(s).and_then(|s| s.tag_node("call_body")))
        })
    })
}
#[inline]
fn parse_function_name(state: Input) -> Output {
    state.rule(BootstrapRule::FunctionName, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "@", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
        })
    })
}
#[inline]
fn parse_call_body(state: Input) -> Output {
    state.rule(BootstrapRule::CallBody, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "(", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.optional(|s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| parse_expression(s).and_then(|s| s.tag_node("expression")))
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| {
                                    s.repeat(0..4294967295, |s| {
                                        s.sequence(|s| {
                                            Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
                                                s.sequence(|s| {
                                                    Ok(s)
                                                        .and_then(|s| builtin_text(s, ",", false))
                                                        .and_then(|s| builtin_ignore(s))
                                                        .and_then(|s| {
                                                            parse_expression(s).and_then(|s| s.tag_node("expression"))
                                                        })
                                                })
                                            })
                                        })
                                    })
                                })
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| s.optional(|s| builtin_text(s, ",", false)))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, ")", false))
        })
    })
}
#[inline]
fn parse_expression(state: Input) -> Output {
    state.rule(BootstrapRule::Expression, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_expression_hard(s).and_then(|s| s.tag_node("expression_hard")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
                                s.sequence(|s| {
                                    Ok(s)
                                        .and_then(|s| builtin_text(s, "|", false))
                                        .and_then(|s| builtin_ignore(s))
                                        .and_then(|s| parse_expression_hard(s).and_then(|s| s.tag_node("expression_hard")))
                                })
                            })
                        })
                    })
                })
        })
    })
}
#[inline]
fn parse_expression_hard(state: Input) -> Output {
    state.rule(BootstrapRule::ExpressionHard, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_expression_soft(s).and_then(|s| s.tag_node("expression_soft")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
                                s.sequence(|s| {
                                    Ok(s)
                                        .and_then(|s| builtin_text(s, "~", false))
                                        .and_then(|s| builtin_ignore(s))
                                        .and_then(|s| parse_expression_soft(s).and_then(|s| s.tag_node("expression_soft")))
                                })
                            })
                        })
                    })
                })
        })
    })
}
#[inline]
fn parse_expression_soft(state: Input) -> Output {
    state.rule(BootstrapRule::ExpressionSoft, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_expression_tag(s).and_then(|s| s.tag_node("expression_tag")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_expression_tag(s).and_then(|s| s.tag_node("expression_tag")))
                        })
                    })
                })
        })
    })
}
#[inline]
fn parse_expression_tag(state: Input) -> Output {
    state.rule(BootstrapRule::ExpressionTag, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.optional(|s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| builtin_text(s, ":", false))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_term(s).and_then(|s| s.tag_node("term")))
        })
    })
}
#[inline]
fn parse_term(state: Input) -> Output {
    state.rule(BootstrapRule::Term, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_prefix(s).and_then(|s| s.tag_node("prefix")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_atomic(s).and_then(|s| s.tag_node("atomic")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_suffix(s).and_then(|s| s.tag_node("suffix")))
                        })
                    })
                })
        })
    })
}
#[inline]
fn parse_negative(state: Input) -> Output {
    state.rule(BootstrapRule::Negative, |s| s.match_string("!", false))
}
#[inline]
fn parse_positive(state: Input) -> Output {
    state.rule(BootstrapRule::Positive, |s| s.match_string("&", false))
}
#[inline]
fn parse_remark(state: Input) -> Output {
    state.rule(BootstrapRule::Remark, |s| s.match_string("^", false))
}
#[inline]
fn parse_prefix(state: Input) -> Output {
    state.rule(BootstrapRule::Prefix, |s| {
        Err(s)
            .or_else(|s| parse_negative(s).and_then(|s| s.tag_node("negative")))
            .or_else(|s| parse_positive(s).and_then(|s| s.tag_node("positive")))
            .or_else(|s| parse_remark(s).and_then(|s| s.tag_node("remark")))
    })
}
#[inline]
fn parse_optional(state: Input) -> Output {
    state.rule(BootstrapRule::Optional, |s| s.match_string("?", false))
}
#[inline]
fn parse_many(state: Input) -> Output {
    state.rule(BootstrapRule::Many, |s| s.match_string("*", false))
}
#[inline]
fn parse_many_1(state: Input) -> Output {
    state.rule(BootstrapRule::Many1, |s| s.match_string("+", false))
}
#[inline]
fn parse_suffix(state: Input) -> Output {
    state.rule(BootstrapRule::Suffix, |s| {
        Err(s)
            .or_else(|s| parse_optional(s).and_then(|s| s.tag_node("optional")))
            .or_else(|s| parse_many(s).and_then(|s| s.tag_node("many")))
            .or_else(|s| parse_many_1(s).and_then(|s| s.tag_node("many_1")))
            .or_else(|s| parse_range_exact(s).and_then(|s| s.tag_node("range_exact")))
            .or_else(|s| parse_range(s).and_then(|s| s.tag_node("range")))
    })
}
#[inline]
fn parse_atomic(state: Input) -> Output {
    state.rule(BootstrapRule::Atomic, |s| {
        Err(s)
            .or_else(|s| parse_group_expression(s).and_then(|s| s.tag_node("group_expression")))
            .or_else(|s| parse_function_call(s).and_then(|s| s.tag_node("function_call")))
            .or_else(|s| parse_boolean(s).and_then(|s| s.tag_node("boolean")))
            .or_else(|s| parse_integer(s).and_then(|s| s.tag_node("integer")))
            .or_else(|s| parse_string_raw(s).and_then(|s| s.tag_node("string_raw")))
            .or_else(|s| parse_string_normal(s).and_then(|s| s.tag_node("string_normal")))
            .or_else(|s| parse_category(s).and_then(|s| s.tag_node("category")))
            .or_else(|s| parse_escaped_unicode(s).and_then(|s| s.tag_node("escaped_unicode")))
            .or_else(|s| parse_regex_embed(s).and_then(|s| s.tag_node("regex_embed")))
            .or_else(|s| parse_regex_range(s).and_then(|s| s.tag_node("regex_range")))
            .or_else(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
    })
}
#[inline]
fn parse_group_expression(state: Input) -> Output {
    state.rule(BootstrapRule::GroupExpression, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "(", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| builtin_text(s, "|", false)))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_expression(s).and_then(|s| s.tag_node("expression")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, ")", false))
        })
    })
}
#[inline]
fn parse_string_raw(state: Input) -> Output {
    state.rule(BootstrapRule::StringRaw, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "'", false))
                .and_then(|s| parse_string_raw_text(s).and_then(|s| s.tag_node("string_raw_text")))
                .and_then(|s| builtin_text(s, "'", false))
        })
    })
}
#[inline]
fn parse_string_raw_text(state: Input) -> Output {
    state.rule(BootstrapRule::StringRawText, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)([^']*)").unwrap())
        })
    })
}
#[inline]
fn parse_string_normal(state: Input) -> Output {
    state.rule(BootstrapRule::StringNormal, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "\"", false))
                .and_then(|s| s.repeat(0..4294967295, |s| parse_string_item(s).and_then(|s| s.tag_node("string_item"))))
                .and_then(|s| builtin_text(s, "\"", false))
        })
    })
}
#[inline]
fn parse_string_item(state: Input) -> Output {
    state.rule(BootstrapRule::StringItem, |s| {
        Err(s)
            .or_else(|s| parse_escaped_unicode(s).and_then(|s| s.tag_node("escaped_unicode")))
            .or_else(|s| parse_escaped_character(s).and_then(|s| s.tag_node("escaped_character")))
            .or_else(|s| parse_text_any(s).and_then(|s| s.tag_node("text_any")))
    })
}
#[inline]
fn parse_escaped_unicode(state: Input) -> Output {
    state.rule(BootstrapRule::EscapedUnicode, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "\\u{", false))
                .and_then(|s| parse_hex(s).and_then(|s| s.tag_node("hex")))
                .and_then(|s| builtin_text(s, "}", false))
        })
    })
}
#[inline]
fn parse_escaped_character(state: Input) -> Output {
    state.rule(BootstrapRule::EscapedCharacter, |s| {
        s.sequence(|s| Ok(s).and_then(|s| builtin_text(s, "\\", false)).and_then(|s| builtin_any(s)))
    })
}
#[inline]
fn parse_hex(state: Input) -> Output {
    state.rule(BootstrapRule::HEX, |s| {
        s.repeat(1..6, |s| {
            s.sequence(|s| {
                Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
                    builtin_regex(s, {
                        static REGEX: OnceLock<Regex> = OnceLock::new();
                        REGEX.get_or_init(|| Regex::new("^(?x)([0-9a-fA-F])").unwrap())
                    })
                })
            })
        })
    })
}
#[inline]
fn parse_text_any(state: Input) -> Output {
    state.rule(BootstrapRule::TextAny, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)([^\"\\\\]+)").unwrap())
        })
    })
}
#[inline]
fn parse_regex_embed(state: Input) -> Output {
    state.rule(BootstrapRule::RegexEmbed, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "/", false))
                .and_then(|s| s.repeat(1..4294967295, |s| parse_regex_item(s).and_then(|s| s.tag_node("regex_item"))))
                .and_then(|s| builtin_text(s, "/", false))
        })
    })
}
#[inline]
fn parse_regex_item(state: Input) -> Output {
    state.rule(BootstrapRule::RegexItem, |s| {
        Err(s)
            .or_else(|s| parse_escaped_character(s).and_then(|s| s.tag_node("escaped_character")))
            .or_else(|s| parse_regex_character(s).and_then(|s| s.tag_node("regex_character")))
    })
}
#[inline]
fn parse_regex_character(state: Input) -> Output {
    state.rule(BootstrapRule::RegexCharacter, |s| {
        s.sequence(|s| Ok(s).and_then(|s| s.lookahead(false, |s| builtin_text(s, "/", false))).and_then(|s| builtin_any(s)))
    })
}
#[inline]
fn parse_regex_range(state: Input) -> Output {
    state.rule(BootstrapRule::RegexRange, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "[", false))
                .and_then(|s| s.optional(|s| parse_regex_negative(s).and_then(|s| s.tag_node("regex_negative"))))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s).and_then(|s| s.lookahead(false, |s| builtin_text(s, "]", false))).and_then(|s| builtin_any(s))
                        })
                    })
                })
                .and_then(|s| builtin_text(s, "]", false))
        })
    })
}
#[inline]
fn parse_regex_negative(state: Input) -> Output {
    state.rule(BootstrapRule::RegexNegative, |s| s.match_string("^", false))
}
#[inline]
fn parse_category(state: Input) -> Output {
    state.rule(BootstrapRule::Category, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.sequence(|s| {
                        Ok(s)
                            .and_then(|s| parse_op_category(s))
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| builtin_text(s, "{", false))
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| {
                                s.optional(|s| {
                                    s.sequence(|s| {
                                        Ok(s)
                                            .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("group")))
                                            .and_then(|s| builtin_ignore(s))
                                            .and_then(|s| builtin_text(s, "=", false))
                                    })
                                })
                            })
                            .and_then(|s| builtin_ignore(s))
                    })
                })
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("script")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "}", false))
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
                            Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
                                s.sequence(|s| {
                                    Ok(s)
                                        .and_then(|s| {
                                            Err(s)
                                                .or_else(|s| builtin_text(s, ".", false))
                                                .or_else(|s| builtin_text(s, "::", false))
                                        })
                                        .and_then(|s| builtin_ignore(s))
                                        .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                                })
                            })
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
                            Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
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
        })
    })
}
#[inline]
fn parse_identifier(state: Input) -> Output {
    state.rule(BootstrapRule::Identifier, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)([_\\p{XID_start}]\\p{XID_continue}*)").unwrap())
        })
    })
}
#[inline]
fn parse_true(state: Input) -> Output {
    state.rule(BootstrapRule::True, |s| s.match_string("true", false))
}
#[inline]
fn parse_false(state: Input) -> Output {
    state.rule(BootstrapRule::False, |s| s.match_string("false", false))
}
#[inline]
fn parse_boolean(state: Input) -> Output {
    state.rule(BootstrapRule::Boolean, |s| {
        Err(s)
            .or_else(|s| parse_true(s).and_then(|s| s.tag_node("true")))
            .or_else(|s| parse_false(s).and_then(|s| s.tag_node("false")))
    })
}
#[inline]
fn parse_integer(state: Input) -> Output {
    state.rule(BootstrapRule::Integer, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(0|[1-9][0-9]*)").unwrap())
        })
    })
}
#[inline]
fn parse_range_exact(state: Input) -> Output {
    state.rule(BootstrapRule::RangeExact, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "{", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_integer(s).and_then(|s| s.tag_node("integer")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "}", false))
        })
    })
}
#[inline]
fn parse_range(state: Input) -> Output {
    state.rule(BootstrapRule::Range, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "{", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_integer(s).and_then(|s| s.tag_node("min"))))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, ",", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_integer(s).and_then(|s| s.tag_node("max"))))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "}", false))
        })
    })
}
#[inline]
fn parse_modifier_call(state: Input) -> Output {
    state.rule(BootstrapRule::ModifierCall, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.lookahead(false, |s| {
                        Err(s)
                            .or_else(|s| parse_kw_class(s))
                            .or_else(|s| parse_kw_union(s))
                            .or_else(|s| parse_kw_group(s))
                            .or_else(|s| parse_kw_macro(s))
                            .or_else(|s| parse_kw_climb(s))
                    })
                })
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
        })
    })
}
#[inline]
fn parse_op_category(state: Input) -> Output {
    state.rule(BootstrapRule::OP_CATEGORY, |s| s.match_string("\\p", false))
}
#[inline]
fn parse_parser(state: Input) -> Output {
    state.rule(BootstrapRule::Parser, |s| s.match_string("parser", false))
}
#[inline]
fn parse_inspector(state: Input) -> Output {
    state.rule(BootstrapRule::Inspector, |s| s.match_string("inspector", false))
}
#[inline]
fn parse_external(state: Input) -> Output {
    state.rule(BootstrapRule::External, |s| s.match_string("external", false))
}
#[inline]
fn parse_kw_external(state: Input) -> Output {
    state.rule(BootstrapRule::KW_EXTERNAL, |s| {
        Err(s)
            .or_else(|s| parse_parser(s).and_then(|s| s.tag_node("parser")))
            .or_else(|s| parse_inspector(s).and_then(|s| s.tag_node("inspector")))
            .or_else(|s| parse_external(s).and_then(|s| s.tag_node("external")))
    })
}
#[inline]
fn parse_kw_grammar(state: Input) -> Output {
    state.rule(BootstrapRule::KW_GRAMMAR, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(grammar)").unwrap())
        })
    })
}
#[inline]
fn parse_kw_import(state: Input) -> Output {
    state.rule(BootstrapRule::KW_IMPORT, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(using|use|import)").unwrap())
        })
    })
}
#[inline]
fn parse_kw_class(state: Input) -> Output {
    state.rule(BootstrapRule::KW_CLASS, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(class|struct)").unwrap())
        })
    })
}
#[inline]
fn parse_kw_union(state: Input) -> Output {
    state.rule(BootstrapRule::KW_UNION, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(union|enum)").unwrap())
        })
    })
}
#[inline]
fn parse_kw_group(state: Input) -> Output {
    state.rule(BootstrapRule::KW_GROUP, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(group|token)").unwrap())
        })
    })
}
#[inline]
fn parse_kw_climb(state: Input) -> Output {
    state.rule(BootstrapRule::KW_CLIMB, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(climb)").unwrap())
        })
    })
}
#[inline]
fn parse_kw_macro(state: Input) -> Output {
    state.rule(BootstrapRule::KW_MACRO, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(macro|def|function|func|fun|fn)").unwrap())
        })
    })
}
#[inline]
fn parse_white_space(state: Input) -> Output {
    state.rule(BootstrapRule::WhiteSpace, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(\\p{White_Space}+)").unwrap())
        })
    })
}
#[inline]
fn parse_comment(state: Input) -> Output {
    state.rule(BootstrapRule::Comment, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(//[^\\n\\r]*)").unwrap())
        })
    })
}

/// All rules ignored in ast mode, inline is not recommended
fn builtin_ignore(state: Input) -> Output {
    state.repeat(0..u32::MAX, |s| parse_white_space(s).or_else(|s| parse_comment(s)))
}

fn builtin_any(state: Input) -> Output {
    state.rule(BootstrapRule::HiddenText, |s| s.match_char_if(|_| true))
}

fn builtin_text<'i>(state: Input<'i>, text: &'static str, case: bool) -> Output<'i> {
    state.rule(BootstrapRule::HiddenText, |s| s.match_string(text, case))
}

fn builtin_regex<'i, 'r>(state: Input<'i>, regex: &'r Regex) -> Output<'i> {
    state.rule(BootstrapRule::HiddenText, |s| s.match_regex(regex))
}
