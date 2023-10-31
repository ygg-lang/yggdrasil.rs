use super::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum YggdrasilErrorSource {
    /// Unknown source
    Missing,
    /// Source from text
    Text(String),
    /// Local path buffer, note path is not core type
    Local(String),
    /// A url path pointer to remote resource
    Remote(String),
}

impl Default for YggdrasilErrorSource {
    fn default() -> Self {
        Self::Missing
    }
}

#[cfg(debug_assertions)]
impl YggdrasilErrorSource {
    /// Convert offset to 1-index line and column.
    ///
    /// Note that this is an O(n) method, only used for quick debugging.
    ///
    /// Please actually use the pre-cached TextSpan method.
    pub fn offset_to_position(text: &str, offset: usize) -> (usize, usize) {
        let mut line = 1;
        let mut column = 1;
        for (i, c) in text.chars().enumerate() {
            if i == offset {
                break;
            }
            if c == '\n' {
                line += 1;
                column = 0;
            }
            else {
                column += 1;
            }
        }
        (line, column)
    }
}
