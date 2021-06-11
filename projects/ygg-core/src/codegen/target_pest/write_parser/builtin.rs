use super::*;

pub fn sealed_unicode(f: &mut impl Write, fn_name: &str) -> std::fmt::Result {
    writeln!(
        f,
        r#"
#[inline]
fn {name}(state: RuleState) -> RuleResult {{
    state.match_char_by(pest::unicode::{name})
}}
"#,
        name = fn_name
    )
}

// #[inline]
// pub fn SKIP(state: RuleState) -> RuleResult {
//     match state.atomicity() == NonAtomic {
//         true => state.sequence(|state| state.repeat(|state| self::WHITESPACE(state))),
//         false => Ok(state)
//     }
// }
pub fn sealed_skip(f: &mut impl Write) -> std::fmt::Result {
    writeln!(
        f,
        r#"
#[inline]
pub fn SKIP(state: RuleState) -> RuleResult {{
    match state.atomicity() == NonAtomic {{
        true => state.repeat(|state| self::WHITESPACE(state)),
        false => Ok(state)
    }}
}}
"#,
    )
}

pub fn sealed_final(f: &mut impl Write) -> std::fmt::Result {
    writeln!(
        f,
        r#"
#[inline]
pub fn ANY(state: RuleState) -> RuleResult {{
    state.skip(1)
}}

#[inline]
pub fn EOI(state: RuleState) -> RuleResult {{
    state.rule(Rule::EOI, |state| state.end_of_input())
}}

#[inline]
pub fn SOI(state: RuleState) -> RuleResult {{
    state.start_of_input()
}}
"#,
    )
}
