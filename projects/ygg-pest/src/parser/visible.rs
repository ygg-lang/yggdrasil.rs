#![allow(non_snake_case, unused_variables)]

use super::*;

#[inline]
pub fn program(state: RuleState) -> RuleResult {
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
pub fn statement(state: RuleState) -> RuleResult {
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
pub fn empty_statement(state: RuleState) -> RuleResult {
    state.rule(Rule::empty_statement, |state| self::eos(state))
}

#[inline]
pub fn eos(state: RuleState) -> RuleResult {
    state.rule(Rule::eos, |state| {
        state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(";"))
    })
}

#[inline]
pub fn grammar_statement(state: RuleState) -> RuleResult {
    state.rule(Rule::grammar_statement, |state| self::grammar(state))
}

#[inline]
pub fn grammar(state: RuleState) -> RuleResult {
    state.match_string("grammar!")
}

#[inline]
pub fn import_statement(state: RuleState) -> RuleResult {
    state.rule(Rule::import_statement, |state| self::import(state))
}

#[inline]
pub fn import(state: RuleState) -> RuleResult {
    state.match_string("import!")
}

#[inline]
pub fn SYMBOL(state: RuleState) -> RuleResult {
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
                                        super::hidden::skip(state)
                                            .and_then(|state| self::XID_CONTINUE(state))
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
pub fn EOI(state: RuleState) -> RuleResult {
    state.rule(Rule::EOI, |state| state.end_of_input())
}

#[inline]
pub fn SOI(state: RuleState) -> RuleResult {
    state.start_of_input()
}

#[inline]
fn XID_CONTINUE(state: RuleState) -> RuleResult {
    state.match_char_by(pest::unicode::XID_CONTINUE)
}

#[inline]
fn XID_START(state: RuleState) -> RuleResult {
    state.match_char_by(pest::unicode::XID_START)
}
