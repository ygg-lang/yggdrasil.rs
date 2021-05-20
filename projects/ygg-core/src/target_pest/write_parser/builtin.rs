use super::*;


pub fn sealed_unicode(f: impl Write, fn_name: &str) -> std::fmt::Result {
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

pub fn sealed_final(f: impl Write) -> std::fmt::Result {
    writeln!(
        f,
        r#"
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
