use std::{collections::BTreeMap, lazy::SyncLazy};

pub static BUILTIN_RULES: SyncLazy<BTreeMap<&'static str, &'static str>> = SyncLazy::new(|| {
    let mut rules = BTreeMap::default();
    rules.insert("ASCII", "[\x00-\x7F]");
    rules.insert("ASCII_ALPHA_DIGIT", "[0-9a-zA-Z]");
    rules.insert("ASCII_UPPERCASE", "[A-Z]");
    rules.insert("ASCII_LOWERCASE", "[a-z]");
    rules.insert("ASCII_DIGIT", "[0-9]");
    rules.insert("ASCII_BIN", "[0-1]");
    rules.insert("ASCII_OCT", "[0-7]");
    rules.insert("ASCII_HEX", "[0-9a-fA-F]");
    //
    rules.insert("ASCII_DIGITS_WLZ", "0|[1-9][0-9]*");
    rules.insert("ASCII_BIN_BYTES", "0[bB][0-1]+");
    rules.insert("ASCII_OCT_BYTES", "0[oO][0-7]+");
    rules.insert("ASCII_HEX_BYTES", "0[xX][0-9a-fA-F]+");
    //
    rules.insert("ASCII_WHITESPACE", "[ \t]");
    rules.insert("ASCII_NEWLINE", "\r?\n");
    return rules;
});
