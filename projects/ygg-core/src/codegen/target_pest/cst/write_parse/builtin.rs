use super::*;

// sealed_unicode(f, "XID_START")?;
// sealed_unicode(f, "XID_CONTINUE")?;
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
