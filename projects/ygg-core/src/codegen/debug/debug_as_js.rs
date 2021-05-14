use std::{
    collections::HashMap,
    fmt::{self, Formatter},
    lazy::SyncLazy,
};

pub const PREDEFINED_FUNCTION_MAP: SyncLazy<HashMap<&str, String>> = SyncLazy::new(|| {
    let mut map = HashMap::default();
    map.insert("interleave", String::from(FUNCTION_INTERLEAVE));
    return map;
});

const FUNCTION_INTERLEAVE: &str = r#"
function interleave(rule, sep, trailing) {
    if (trailing > 0) {
        // must add trailing separator
        return seq(rule, repeat(seq(sep, rule)), sep)
    }
    else if (trailing < 0) {
        // disallow add trailing separator
        return seq(rule, repeat(seq(sep, rule)))
    }
    else {
        // trailing separator is optional
        return seq(rule, repeat(seq(sep, rule)), optional(sep))
    }
}
"#;

struct T;

pub trait DebugJS {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result;
}

impl DebugJS for T {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "module.exports = grammar({{")?;
        writeln!(f, "module.exports = grammar({{")?;
        Ok(())
    }
}
