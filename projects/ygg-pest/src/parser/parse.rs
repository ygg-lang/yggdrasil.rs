use super::*;

#[inline]
pub fn program(state: RuleState) -> RuleResult {
    state.rule(Rule::program, |state| state.sequence(|state| state.optional(|state| self::statement(state).and_then(|state| state.repeat(|state| state.sequence(|state| SKIP(state).and_then(|state| self::statement(state))))))))
}

#[inline]
pub fn statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
    state.rule(Rule::statement, |state| {
        self::empty_statement(state)
            .or_else(|state| state.sequence(|state| self::grammar_statement(state).and_then(|state| SKIP(state)).and_then(|state| state.optional(|state| self::eos(state)))))
            .or_else(|state| state.sequence(|state| self::import_statement(state).and_then(|state| SKIP(state)).and_then(|state| state.optional(|state| self::eos(state)))))
    })
}

#[inline]
pub fn empty_statement(state: RuleState) -> RuleResult {
    state.rule(Rule::empty_statement, |state| self::eos(state))
}

#[inline]
pub fn eos(state: RuleState) -> RuleResult {
    state.rule(Rule::eos, |state| state.atomic(Atomic, |state| state.match_string(";")))
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
pub fn prefix(state: RuleState) -> RuleResult {
    state.rule(Rule::prefix, |state| state.match_char_by(|c| matches!(c, '!' | '^')))
}

#[inline]
pub fn SYMBOL(state: RuleState) -> RuleResult {
    state.rule(Rule::SYMBOL, |state| {
        state.sequence(|state| {
            self::XID_START(state)
                .and_then(|state| SKIP(state))
                .and_then(|state| state.sequence(|state| state.optional(|state| self::XID_CONTINUE(state).and_then(|state| state.repeat(|state| state.sequence(|state| SKIP(state).and_then(|state| self::XID_CONTINUE(state))))))))
        })
    })
}

#[inline]
fn XID_CONTINUE(state: RuleState) -> RuleResult {
    state.match_char_by(pest::unicode::XID_CONTINUE)
}

#[inline]
fn XID_START(state: RuleState) -> RuleResult {
    state.match_char_by(pest::unicode::XID_START)
}

#[inline]
pub fn SKIP(state: RuleState) -> RuleResult {
    match state.atomicity() == NonAtomic {
        true => state.sequence(|state| state.repeat(|state| self::WHITESPACE(state))),
        false => Ok(state),
    }
}

#[inline]
pub fn WHITESPACE(state: RuleState) -> RuleResult {
    state.skip(1)
}

#[inline]
pub fn ANY(state: RuleState) -> RuleResult {
    state.skip(1)
}

#[inline]
pub fn EOI(state: RuleState) -> RuleResult {
    state.rule(Rule::EOI, |state| state.end_of_input())
}

#[inline]
pub fn SOI(state: RuleState) -> RuleResult {
    state.start_of_input()
}
