// pub(crate) mod cst_mode;
pub(crate) mod text_index;

pub mod state;

/// A string that is surrounded by other strings.
/// ```ygg
/// r#" content "#
/// ```
pub struct SurroundedString<'i> {
    pub start: CapturedString<'i>,
    pub content: CapturedString<'i>,
    pub end: CapturedString<'i>,
}

pub struct CapturedString<'i> {
    /// The string that was captured.
    pub string: &'i str,
    /// The offset of the string in the original string.
    pub offset: usize,
}

impl<'i> CapturedString<'i> {
    pub fn new(string: &'i str, offset: usize) -> Self {
        Self { string, offset }
    }
    pub fn slice(string: &'i str, start: usize, length: usize) -> Self {
        Self { string: &string[start..start + length], offset: start }
    }
}
