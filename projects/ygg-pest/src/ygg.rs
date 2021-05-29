pub struct YGGParser;
#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    EOI,
    program,
    statement,
    empty_statement,
    eos,
    grammar_statement,
    grammar,
    fragment_statement,
    fragment_pair,
    fragment,
    import_statement,
    import,
    ignore_statement,
    ignore,
    assign_statement,
    assign_kind,
    expression,
    expr,
    term,
    infix,
    prefix,
    suffix,
    apply,
    apply_kv,
    data,
    slice,
    list,
    string,
    integer,
    SpecialValue,
    comment_doc,
    comment_s_l,
    comment_m_l,
    COMMENT,
    symbol_path,
    SYMBOL,
    WHITESPACE,
    WHITE_SPACE,
    NEWLINE,
}
#[allow(clippy::all)]
impl ::pest::Parser<Rule> for YGGParser {
    fn parse<'i>(rule: Rule, input: &'i str) -> ::std::result::Result<::pest::iterators::Pairs<'i, Rule>, ::pest::error::Error<Rule>> {
        mod rules {
            pub mod hidden {
                use super::super::Rule;
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn skip(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    if state.atomicity() == ::pest::Atomicity::NonAtomic {
                        state.sequence(|state| state.repeat(|state| super::visible::WHITESPACE(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::visible::COMMENT(state).and_then(|state| state.repeat(|state| super::visible::WHITESPACE(state)))))))
                    } else {
                        Ok(state)
                    }
                }
            }
            pub mod visible {
                use super::super::Rule;
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn program(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::program, |state| {
                        state.sequence(|state| {
                            self::SOI(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.sequence(|state| state.optional(|state| self::statement(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::statement(state))))))))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::EOI(state))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::statement, |state| {
                        self::empty_statement(state)
                            .or_else(|state| state.sequence(|state| self::grammar_statement(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))
                            .or_else(|state| state.sequence(|state| self::fragment_statement(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))
                            .or_else(|state| state.sequence(|state| self::ignore_statement(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))
                            .or_else(|state| state.sequence(|state| self::assign_statement(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))
                            .or_else(|state| self::comment_doc(state))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn empty_statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::empty_statement, |state| self::eos(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn eos(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::eos, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(";")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn grammar_statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::grammar_statement, |state| {
                        state.sequence(|state| self::grammar(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::string(state))).or_else(|state| {
                            state.sequence(|state| {
                                self::grammar(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("{")
                                                .and_then(|state| super::hidden::skip(state))
                                                .and_then(|state| state.sequence(|state| state.optional(|state| self::string(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::string(state))))))))
                                                .and_then(|state| super::hidden::skip(state))
                                                .and_then(|state| state.match_string("}"))
                                        })
                                        .or_else(|state| {
                                            state.sequence(|state| {
                                                state
                                                    .match_string("[")
                                                    .and_then(|state| super::hidden::skip(state))
                                                    .and_then(|state| state.sequence(|state| state.optional(|state| self::string(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::string(state))))))))
                                                    .and_then(|state| super::hidden::skip(state))
                                                    .and_then(|state| state.match_string("]"))
                                            })
                                        })
                                })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn grammar(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::grammar, |state| state.match_string("grammar!"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn fragment_statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::fragment_statement, |state| {
                        state.sequence(|state| {
                            self::fragment(state).and_then(|state| super::hidden::skip(state)).and_then(|state| {
                                self::SYMBOL(state).or_else(|state| {
                                    state.sequence(|state| {
                                        self::SYMBOL(state)
                                            .and_then(|state| super::hidden::skip(state))
                                            .and_then(|state| state.match_string("{"))
                                            .and_then(|state| super::hidden::skip(state))
                                            .and_then(|state| state.sequence(|state| state.optional(|state| self::SYMBOL(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::SYMBOL(state))))))))
                                            .and_then(|state| super::hidden::skip(state))
                                            .and_then(|state| state.match_string("}"))
                                    })
                                })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn fragment_pair(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| {
                        self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(":")).and_then(|state| super::hidden::skip(state)).and_then(|state| {
                            self::SYMBOL(state).or_else(|state| {
                                state.sequence(|state| {
                                    state
                                        .match_string("[")
                                        .and_then(|state| super::hidden::skip(state))
                                        .and_then(|state| state.sequence(|state| state.optional(|state| self::SYMBOL(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::SYMBOL(state))))))))
                                        .and_then(|state| super::hidden::skip(state))
                                        .and_then(|state| state.match_string("]"))
                                })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn fragment(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::fragment, |state| state.match_string("fragment!"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn import_statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::import_statement, |state| {
                        state.sequence(|state| self::import(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::string(state))).or_else(|state| {
                            state.sequence(|state| {
                                self::import(state)
                                    .and_then(|state| super::hidden::skip(state))
                                    .and_then(|state| self::string(state))
                                    .and_then(|state| super::hidden::skip(state))
                                    .and_then(|state| state.match_string("{"))
                                    .and_then(|state| super::hidden::skip(state))
                                    .and_then(|state| {
                                        state.match_string("}").or_else(|state| {
                                            state.sequence(|state| {
                                                self::SYMBOL(state)
                                                    .and_then(|state| super::hidden::skip(state))
                                                    .and_then(|state| {
                                                        state.sequence(|state| {
                                                            state.optional(|state| {
                                                                state
                                                                    .sequence(|state| state.match_string(",").and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)))
                                                                    .and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.match_string(",").and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)))))))
                                                            })
                                                        })
                                                    })
                                                    .and_then(|state| super::hidden::skip(state))
                                                    .and_then(|state| state.optional(|state| state.match_string(",")))
                                                    .and_then(|state| super::hidden::skip(state))
                                                    .and_then(|state| state.match_string("}"))
                                            })
                                        })
                                    })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn import(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::import, |state| state.match_string("import!"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ignore_statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::ignore_statement, |state| {
                        state
                            .sequence(|state| self::ignore(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)))
                            .or_else(|state| state.sequence(|state| self::ignore(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("{")).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))
                            .or_else(|state| {
                                state.sequence(|state| {
                                    self::ignore(state)
                                        .and_then(|state| super::hidden::skip(state))
                                        .and_then(|state| state.match_string("{"))
                                        .and_then(|state| super::hidden::skip(state))
                                        .and_then(|state| self::SYMBOL(state))
                                        .and_then(|state| super::hidden::skip(state))
                                        .and_then(|state| {
                                            state.sequence(|state| {
                                                state.optional(|state| {
                                                    state
                                                        .sequence(|state| state.match_string(",").and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)))
                                                        .and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.match_string(",").and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)))))))
                                                })
                                            })
                                        })
                                        .and_then(|state| super::hidden::skip(state))
                                        .and_then(|state| state.optional(|state| state.match_string(",")))
                                        .and_then(|state| super::hidden::skip(state))
                                        .and_then(|state| state.match_string("}"))
                                })
                            })
                            .or_else(|state| {
                                state.sequence(|state| {
                                    self::ignore(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("[")).and_then(|state| super::hidden::skip(state)).and_then(|state| {
                                        state.match_string("]").or_else(|state| {
                                            state.sequence(|state| {
                                                self::SYMBOL(state)
                                                    .and_then(|state| super::hidden::skip(state))
                                                    .and_then(|state| {
                                                        state.sequence(|state| {
                                                            state.optional(|state| {
                                                                state
                                                                    .sequence(|state| state.match_string(",").and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)))
                                                                    .and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.match_string(",").and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)))))))
                                                            })
                                                        })
                                                    })
                                                    .and_then(|state| super::hidden::skip(state))
                                                    .and_then(|state| state.optional(|state| state.match_string(",")))
                                                    .and_then(|state| super::hidden::skip(state))
                                                    .and_then(|state| state.match_string("]"))
                                            })
                                        })
                                    })
                                })
                            })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ignore(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::ignore, |state| state.match_string("ignore!"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn assign_statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::assign_statement, |state| {
                        state.sequence(|state| {
                            self::SYMBOL(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::assign_kind(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.optional(|state| state.match_string("|")))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::expression(state))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn assign_kind(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::assign_kind, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.repeat(|state| state.match_string("^").or_else(|state| state.match_string("_")).or_else(|state| state.match_string("@"))).and_then(|state| state.match_string("="))))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn expression(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::expression, |state| state.sequence(|state| self::expr(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn expr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| {
                        state.rule(Rule::expr, |state| {
                            state.sequence(|state| {
                                self::term(state).and_then(|state| super::hidden::skip(state)).and_then(|state| {
                                    state.sequence(|state| {
                                        state.optional(|state| {
                                            state
                                                .sequence(|state| self::infix(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::term(state)))
                                                .and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::infix(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::term(state)))))))
                                        })
                                    })
                                })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn term(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::term, |state| {
                        state.sequence(|state| {
                            state
                                .sequence(|state| state.optional(|state| self::prefix(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::prefix(state)))))))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| {
                                    state
                                        .sequence(|state| state.match_string("(").and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")")))
                                        .or_else(|state| self::data(state))
                                })
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| {
                                    state.sequence(|state| {
                                        state.optional(|state| {
                                            self::suffix(state)
                                                .or_else(|state| self::slice(state))
                                                .or_else(|state| self::apply(state))
                                                .and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::suffix(state).or_else(|state| self::slice(state)).or_else(|state| self::apply(state))))))
                                        })
                                    })
                                })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn infix(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::infix, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("~")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn prefix(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::prefix, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("!").or_else(|state| state.match_string("^"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn suffix(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::suffix, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("?").or_else(|state| state.match_string("+")).or_else(|state| state.match_string("*"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn apply(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| {
                        state.rule(Rule::apply, |state| {
                            state.sequence(|state| state.match_string("@").and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("(")).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")"))).or_else(|state| {
                                state.sequence(|state| {
                                    state
                                        .match_string("(")
                                        .and_then(|state| super::hidden::skip(state))
                                        .and_then(|state| self::apply_kv(state))
                                        .and_then(|state| super::hidden::skip(state))
                                        .and_then(|state| {
                                            state.sequence(|state| {
                                                state.optional(|state| {
                                                    state
                                                        .sequence(|state| state.match_string(",").and_then(|state| super::hidden::skip(state)).and_then(|state| self::apply_kv(state)))
                                                        .and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.match_string(",").and_then(|state| super::hidden::skip(state)).and_then(|state| self::apply_kv(state)))))))
                                                })
                                            })
                                        })
                                        .and_then(|state| super::hidden::skip(state))
                                        .and_then(|state| state.optional(|state| state.match_string(",")))
                                        .and_then(|state| super::hidden::skip(state))
                                        .and_then(|state| state.match_string(")"))
                                })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn apply_kv(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::apply_kv, |state| {
                        state
                            .sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(":")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)))
                            .or_else(|state| self::expr(state))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn data(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::data, |state| self::symbol_path(state).or_else(|state| self::list(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn slice(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::slice, |state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn list(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::list, |state| {
                        state.sequence(|state| {
                            state.match_string("[").and_then(|state| super::hidden::skip(state)).and_then(|state| {
                                state.match_string("]").or_else(|state| {
                                    state.sequence(|state| {
                                        self::data(state)
                                            .and_then(|state| super::hidden::skip(state))
                                            .and_then(|state| {
                                                state.sequence(|state| {
                                                    state.optional(|state| {
                                                        state
                                                            .sequence(|state| state.match_string(",").and_then(|state| super::hidden::skip(state)).and_then(|state| self::data(state)))
                                                            .and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.match_string(",").and_then(|state| super::hidden::skip(state)).and_then(|state| self::data(state)))))))
                                                    })
                                                })
                                            })
                                            .and_then(|state| super::hidden::skip(state))
                                            .and_then(|state| state.optional(|state| state.match_string(",")))
                                            .and_then(|state| super::hidden::skip(state))
                                            .and_then(|state| state.match_string("]"))
                                    })
                                })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn string(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::string, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| {
                            state
                                .sequence(|state| {
                                    state
                                        .match_string("'")
                                        .and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| state.match_string("'")).and_then(|state| self::ANY(state))).or_else(|state| state.sequence(|state| state.match_string("\\").and_then(|state| self::ANY(state))))))
                                        .and_then(|state| state.match_string("'"))
                                })
                                .or_else(|state| {
                                    state.sequence(|state| {
                                        state
                                            .match_string("\"")
                                            .and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| state.match_string("\"")).and_then(|state| self::ANY(state))).or_else(|state| state.sequence(|state| state.match_string("\\").and_then(|state| self::ANY(state))))))
                                            .and_then(|state| state.match_string("\""))
                                    })
                                })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn integer(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::integer, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| {
                            state
                                .match_string("0")
                                .or_else(|state| state.sequence(|state| self::ASCII_NONZERO_DIGIT(state).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| state.match_string("_")).and_then(|state| self::ASCII_DIGIT(state)))))))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SpecialValue(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SpecialValue, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("true").or_else(|state| state.match_string("false")).or_else(|state| state.match_string("null"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn comment_doc(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::comment_doc, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| {
                            state.sequence(|state| {
                                state
                                    .match_string("//")
                                    .and_then(|state| state.match_string("!").or_else(|state| state.match_string("?")).or_else(|state| state.match_string("*")))
                                    .and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::ANY(state)))))
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn comment_s_l(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::comment_s_l, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.match_string("//").and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::ANY(state)))))))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn comment_m_l(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::comment_m_l, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| {
                            state.sequence(|state| {
                                state
                                    .match_string("/*")
                                    .and_then(|state| state.repeat(|state| self::comment_m_l(state).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.match_string("*/")).and_then(|state| self::ANY(state))))))
                                    .and_then(|state| state.match_string("*/"))
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn COMMENT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::COMMENT, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::comment_s_l(state).or_else(|state| self::comment_m_l(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn symbol_path(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::symbol_path, |state| {
                        state.sequence(|state| {
                            self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| {
                                state.sequence(|state| {
                                    state.optional(|state| {
                                        state
                                            .sequence(|state| state.match_string("::").or_else(|state| state.match_string(".")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)))
                                            .and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.match_string("::").or_else(|state| state.match_string(".")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)))))))
                                    })
                                })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SYMBOL(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SYMBOL, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.match_string("_").or_else(|state| self::XID_START(state)).and_then(|state| state.repeat(|state| self::XID_CONTINUE(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn WHITESPACE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::WHITESPACE, |state| self::COMMENT(state).or_else(|state| self::WHITE_SPACE(state)).or_else(|state| self::NEWLINE(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn WHITE_SPACE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::WHITE_SPACE, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.match_string(" ").or_else(|state| state.match_string("\t")).and_then(|state| state.repeat(|state| state.match_string(" ").or_else(|state| state.match_string("\t"))))))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NEWLINE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::NEWLINE, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| {
                            state.sequence(|state| {
                                state
                                    .match_string("\r\n")
                                    .or_else(|state| state.match_string("\r"))
                                    .or_else(|state| state.match_string("\n"))
                                    .and_then(|state| state.repeat(|state| state.match_string("\r\n").or_else(|state| state.match_string("\r")).or_else(|state| state.match_string("\n"))))
                            })
                        })
                    })
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ANY(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.skip(1)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn EOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EOI, |state| state.end_of_input())
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn SOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.start_of_input()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_NONZERO_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('1'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_CONTINUE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::XID_CONTINUE)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_START(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::XID_START)
                }
            }
            pub use self::visible::*;
        }
        ::pest::state(input, |state| match rule {
            Rule::program => rules::program(state),
            Rule::statement => rules::statement(state),
            Rule::empty_statement => rules::empty_statement(state),
            Rule::eos => rules::eos(state),
            Rule::grammar_statement => rules::grammar_statement(state),
            Rule::grammar => rules::grammar(state),
            Rule::fragment_statement => rules::fragment_statement(state),
            Rule::fragment_pair => rules::fragment_pair(state),
            Rule::fragment => rules::fragment(state),
            Rule::import_statement => rules::import_statement(state),
            Rule::import => rules::import(state),
            Rule::ignore_statement => rules::ignore_statement(state),
            Rule::ignore => rules::ignore(state),
            Rule::assign_statement => rules::assign_statement(state),
            Rule::assign_kind => rules::assign_kind(state),
            Rule::expression => rules::expression(state),
            Rule::expr => rules::expr(state),
            Rule::term => rules::term(state),
            Rule::infix => rules::infix(state),
            Rule::prefix => rules::prefix(state),
            Rule::suffix => rules::suffix(state),
            Rule::apply => rules::apply(state),
            Rule::apply_kv => rules::apply_kv(state),
            Rule::data => rules::data(state),
            Rule::slice => rules::slice(state),
            Rule::list => rules::list(state),
            Rule::string => rules::string(state),
            Rule::integer => rules::integer(state),
            Rule::SpecialValue => rules::SpecialValue(state),
            Rule::comment_doc => rules::comment_doc(state),
            Rule::comment_s_l => rules::comment_s_l(state),
            Rule::comment_m_l => rules::comment_m_l(state),
            Rule::COMMENT => rules::COMMENT(state),
            Rule::symbol_path => rules::symbol_path(state),
            Rule::SYMBOL => rules::SYMBOL(state),
            Rule::WHITESPACE => rules::WHITESPACE(state),
            Rule::WHITE_SPACE => rules::WHITE_SPACE(state),
            Rule::NEWLINE => rules::NEWLINE(state),
            Rule::EOI => rules::EOI(state),
        })
    }
}
