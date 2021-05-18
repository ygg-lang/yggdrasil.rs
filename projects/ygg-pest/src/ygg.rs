pub struct YGGParser;
#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    EOI,
    program_inner,
    program,
    statement,
    empty_statement,
    eos,
    grammar_statement,
    grammar,
    import_statement,
    import,
    SYMBOL,
    WHITESPACE,
}
#[allow(clippy::all)]
impl ::pest::Parser<Rule> for YGGParser {
    fn parse<'i>(
        rule: Rule,
        input: &'i str,
    ) -> ::std::result::Result<::pest::iterators::Pairs<'i, Rule>, ::pest::error::Error<Rule>> {
        mod rules {
            pub mod hidden {
                use super::super::Rule;
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn skip(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    if state.atomicity() == ::pest::Atomicity::NonAtomic {
                        state.repeat(|state| super::visible::WHITESPACE(state))
                    } else {
                        Ok(state)
                    }
                }
            }
            pub mod visible {
                use super::super::Rule;
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn program_inner(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| {
                        self::SOI(state)
                            .and_then(|state| super::hidden::skip(state))
                            .and_then(|state| {
                                state.sequence(|state| {
                                    state.optional(|state| {
                                        self::statement(state).and_then(|state| {
                                            state.repeat(|state| {
                                                state.sequence(|state| {
                                                    super::hidden::skip(state)
                                                        .and_then(|state| self::statement(state))
                                                })
                                            })
                                        })
                                    })
                                })
                            })
                            .and_then(|state| super::hidden::skip(state))
                            .and_then(|state| self::EOI(state))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn program(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::program, |state| {
                        state.sequence(|state| {
                            state.optional(|state| {
                                self::statement(state).and_then(|state| {
                                    state.repeat(|state| {
                                        state.sequence(|state| {
                                            super::hidden::skip(state)
                                                .and_then(|state| self::statement(state))
                                        })
                                    })
                                })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn statement(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::statement, |state| {
                        self::empty_statement(state)
                            .or_else(|state| {
                                state.sequence(|state| {
                                    self::grammar_statement(state)
                                        .and_then(|state| super::hidden::skip(state))
                                        .and_then(|state| state.optional(|state| self::eos(state)))
                                })
                            })
                            .or_else(|state| {
                                state.sequence(|state| {
                                    self::import_statement(state)
                                        .and_then(|state| super::hidden::skip(state))
                                        .and_then(|state| state.optional(|state| self::eos(state)))
                                })
                            })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn empty_statement(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::empty_statement, |state| self::eos(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn eos(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::eos, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(";"))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn grammar_statement(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::grammar_statement, |state| {
                        state.sequence(|state| {
                            self::grammar(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::SYMBOL(state))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn grammar(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("grammar!")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn import_statement(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::import_statement, |state| {
                        state.sequence(|state| {
                            self::import(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::SYMBOL(state))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn import(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("import!")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SYMBOL(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SYMBOL, |state| {
                        state.sequence(|state| {
                            self::XID_START(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| {
                                    state.sequence(|state| {
                                        state.optional(|state| {
                                            self::XID_CONTINUE(state).and_then(|state| {
                                                state.repeat(|state| {
                                                    state.sequence(|state| {
                                                        super::hidden::skip(state).and_then(
                                                            |state| self::XID_CONTINUE(state),
                                                        )
                                                    })
                                                })
                                            })
                                        })
                                    })
                                })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn WHITESPACE(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::WHITESPACE, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| {
                            self::NEWLINE(state).or_else(|state| self::WHITE_SPACE(state))
                        })
                    })
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn EOI(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EOI, |state| state.end_of_input())
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn SOI(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.start_of_input()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn NEWLINE(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state
                        .match_string("\n")
                        .or_else(|state| state.match_string("\r\n"))
                        .or_else(|state| state.match_string("\r"))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn WHITE_SPACE(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::WHITE_SPACE)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_CONTINUE(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::XID_CONTINUE)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_START(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::XID_START)
                }
            }
            pub use self::visible::*;
        }
        ::pest::state(input, |state| match rule {
            Rule::program_inner => rules::program_inner(state),
            Rule::program => rules::program(state),
            Rule::statement => rules::statement(state),
            Rule::empty_statement => rules::empty_statement(state),
            Rule::eos => rules::eos(state),
            Rule::grammar_statement => rules::grammar_statement(state),
            Rule::grammar => rules::grammar(state),
            Rule::import_statement => rules::import_statement(state),
            Rule::import => rules::import(state),
            Rule::SYMBOL => rules::SYMBOL(state),
            Rule::WHITESPACE => rules::WHITESPACE(state),
            Rule::EOI => rules::EOI(state),
        })
    }
}
