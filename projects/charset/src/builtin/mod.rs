use std::{collections::BTreeMap, ops::Range};

pub use generated::*;

mod generated;

pub const XID_START: &'static [Range<char>] = &['a'..'c', 'c'..'c'];

pub const WHITE_SPACE: &'static [Range<char>] = &['a'..'c', 'c'..'c'];

pub const LETTER: &'static [Range<char>] = &['a'..'c', 'c'..'c'];

// pub static BUILTIN_CHARACTER_SETS: LazyLock<BTreeMap<&'static str, &'static [Range<char>]>> = LazyLock::new(|| {
//     let mut out = BTreeMap::new();
//     out.insert("XID_START", &['a'..'c']);
//     out
// });
#[cfg(feature = "lazy_static")]
lazy_static::lazy_static! {
    pub static ref BUILTIN_CHARACTER_RANGES: BTreeMap<&'static str, &'static [Range<char>]> = {
        let mut out: BTreeMap<&'static str, &'static [Range<char>]> = BTreeMap::new();
        out.insert("XID_START", XID_START);
        out.insert("XID_START", XID_START);
        out.insert("XID_CONTINUE", XID_CONTINUE);
        out
    };
}
pub static BUILTIN_CHARACTER_RANGES: BTreeMap<&'static str, &'static [Range<char>]> = {
    let mut out: BTreeMap<&'static str, &'static [Range<char>]> = BTreeMap::new();
    out.insert("L", LETTER);
    out.insert("LETTER", LETTER);
    out.insert("XID_START", XID_START);
    out.insert("XID_CONTINUE", XID_CONTINUE);
    out
};

/// https://www.regular-expressions.info/unicode.html#prop
pub const REGEX_CHARACTER_RANGES: &'static [(&'static str, &'static str, &'static str, &'static [Range<char>])] = &[
    // Letter
    ("L", "Letter", "Letter", LETTER),
    ("Ll", "Lowercase_Letter", "LowercaseLetter", LETTER),
    ("Lu", "Uppercase_Letter", "UppercaseLetter", LETTER),
    ("Lt", "Titlecase_Letter", "TitlecaseLetter", LETTER),
    ("L&", "Cased_Letter", "CasedLetter", LETTER),
    ("Lm", "Modifier_Letter", "ModifierLetter", LETTER),
    ("Lo", "Other_Letter", "OtherLetter", LETTER),
    // Mark
    ("M", "Mark", "Mark", LETTER),
    ("Mn", "Non_Spacing_Mark", "Non_Spacing_Mark", LETTER),
    ("Mc", "Spacing_Combining_Mark", "Spacing_Combining_Mark", LETTER),
    ("Me", "Enclosing_Mark", "Enclosing_Mark", LETTER),
    // Separator
    ("Z", "Separator", "Separator", LETTER),
    ("Zs", "Space_Separator", "Space_Separator", LETTER),
    ("Zl", "Line_Separator", "Line_Separator", LETTER),
    ("Zp", "Paragraph_Separator", "Paragraph_Separator", LETTER),
    // Symbol
    ("S", "Symbol", "Symbol", LETTER),
    ("Sm", "Math_Symbol", "Math_Symbol", LETTER),
    ("Sc", "Currency_Symbol", "Currency_Symbol", LETTER),
    ("Sk", "Modifier_Symbol", "Modifier_Symbol", LETTER),
    ("So", "Other_Symbol", "Other_Symbol", LETTER),
    // Number
    ("N", "Number", "Number", LETTER),
    ("Nd", "Decimal_Digit_Number", "Decimal_Digit_Number", LETTER),
    ("Nl", "Letter_Number", "Letter_Number", LETTER),
    ("No", "Other_Number", "Other_Number", LETTER),
    // Punctuation
    ("P", "Punctuation", "Punctuation", LETTER),
    ("Pd", "Dash_Punctuation", "Dash_Punctuation", LETTER),
    ("Ps", "Open_Punctuation", "Open_Punctuation", LETTER),
    ("Pe", "Close_Punctuation", "Close_Punctuation", LETTER),
    ("Pi", "Initial_Punctuation", "Initial_Punctuation", LETTER),
    ("Pf", "Final_Punctuation", "Final_Punctuation", LETTER),
    ("Pc", "Connector_Punctuation", "Connector_Punctuation", LETTER),
    ("Po", "Other_Punctuation", "Other_Punctuation", LETTER),
    // Other
    ("C", "Other", "Other", LETTER),
    ("Cc", "Control", "Control", LETTER),
    ("Cf", "Format", "Format", LETTER),
    ("Co", "Private_Use", "Private_Use", LETTER),
    ("Cs", "Surrogate", "Surrogate", LETTER),
    ("Cn", "Unassigned", "Unassigned", LETTER),
];
