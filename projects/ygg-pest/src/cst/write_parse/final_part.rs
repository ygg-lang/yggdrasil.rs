use super::*;

impl PestCSTW {
    pub(super) fn write_final(&self, f: &mut impl Write) -> std::fmt::Result {
        writeln!(f,"// region Final")?;
        writeln!(f)?;
        self.write_skip(f)?;
        writeln!(f)?;
        sealed_final(f)?;
        writeln!(f)?;
        writeln!(f,"// endregion")?;
        Ok(())
    }
    fn write_skip(&self, f: &mut impl Write) -> std::fmt::Result {
        match self.ignores.len() {
            0 => skip_none(f),
            _ => {
                skip_some(f)?;
                writeln!(
                    f,
                    r#"
#[inline]
pub fn IGNORE(s: RuleState) -> RuleResult {{
    match cfg!(feature = "no-ignored") {{
        true => s.atomic(Atomicity::CompoundAtomic, ignore_terms!({term})),
        false => s.atomic(Atomicity::CompoundAtomic, |s| s.rule(Rule::IGNORE, ignore_terms!({term}))),
    }}
}}
"#,
                    term = self.ignores.join(", ")
                )
            }
        }
    }
}

pub fn skip_none(f: &mut impl Write) -> std::fmt::Result {
    writeln!(
        f,
        r#"
#[inline]
pub fn SKIP(state: RuleState) -> RuleResult {{
    Ok(state)
}}
"#,
    )
}

pub fn skip_some(f: &mut impl Write) -> std::fmt::Result {
    writeln!(
        f,
        r#"
#[inline]
pub fn SKIP(state: RuleState) -> RuleResult {{
    match state.atomicity() == Atomicity::NonAtomic {{
        true => state.repeat(|state| self::IGNORE(state)),
        false => Ok(state),
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
pub fn UNNAMED<'i>(s: RuleState<'i>, input: &'i str) -> RuleResult<'i> {{
    match cfg!(feature = "no-unnamed") {{
        true => s.match_string(input),
        false => s.rule(Rule::UNNAMED, |s| s.match_string(input)),
    }}
}}

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
