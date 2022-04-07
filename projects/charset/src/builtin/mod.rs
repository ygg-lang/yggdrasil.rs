use std::ops::{Range, RangeInclusive};

pub use generated::*;

mod generated;

pub static BUILTIN_CHARACTER_RANGES: &'static [(&'static str, &'static str, &'static [RangeInclusive<char>])] = &[
    // Letter
    ("XID_START", "XID_Start", LETTER),
];

/// https://www.regular-expressions.info/unicode.html#prop
pub const REGEX_CHARACTER_RANGES: &'static [(&'static str, &'static str, &'static [RangeInclusive<char>])] = &[
    // Letter
    ("L", "Letter", LETTER),
    ("Ll", "Lowercase_Letter", LETTER),
    ("Lu", "Uppercase_Letter", LETTER),
    ("Lt", "Titlecase_Letter", LETTER),
    ("L&", "Cased_Letter", LETTER),
    ("Lm", "Modifier_Letter", LETTER),
    ("Lo", "Other_Letter", LETTER),
    // Mark
    ("M", "Mark", LETTER),
    ("Mn", "Non_Spacing_Mark", LETTER),
    ("Mc", "Spacing_Combining_Mark", LETTER),
    ("Me", "Enclosing_Mark", LETTER),
    // Separator
    ("Z", "Separator", LETTER),
    ("Zs", "Space_Separator", LETTER),
    ("Zl", "Line_Separator", LETTER),
    ("Zp", "Paragraph_Separator", LETTER),
    // Symbol
    ("S", "Symbol", LETTER),
    ("Sm", "Math_Symbol", LETTER),
    ("Sc", "Currency_Symbol", LETTER),
    ("Sk", "Modifier_Symbol", LETTER),
    ("So", "Other_Symbol", LETTER),
    // Number
    ("N", "Number", LETTER),
    ("Nd", "Decimal_Digit_Number", LETTER),
    ("Nl", "Letter_Number", LETTER),
    ("No", "Other_Number", LETTER),
    // Punctuation
    ("P", "Punctuation", LETTER),
    ("Pd", "Dash_Punctuation", LETTER),
    ("Ps", "Open_Punctuation", LETTER),
    ("Pe", "Close_Punctuation", LETTER),
    ("Pi", "Initial_Punctuation", LETTER),
    ("Pf", "Final_Punctuation", LETTER),
    ("Pc", "Connector_Punctuation", LETTER),
    ("Po", "Other_Punctuation", LETTER),
    // Other
    ("C", "Other", LETTER),
    ("Cc", "Control", LETTER),
    ("Cf", "Format", LETTER),
    ("Co", "Private_Use", LETTER),
    ("Cs", "Surrogate", LETTER),
    ("Cn", "Unassigned", LETTER),
];
